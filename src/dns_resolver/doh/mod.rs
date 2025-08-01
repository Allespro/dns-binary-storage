use ureq;
use sonic_rs::{Deserialize};

// cloudflare-dns.com

#[derive(Deserialize)]
pub struct DNSResponse {
    #[serde(rename = "Answer")]
    pub answer: Vec<Answer>,
}

#[derive(Deserialize)]
pub struct Answer {
    pub data: String,
}

pub fn resolve(domain: &str, resolver: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body: String = ureq::get(resolver)
        .query("name", domain)
        .query("type", "TXT")
        .header("accept", "application/dns-json")
        .call()?.body_mut().read_to_string()?;

    let p: DNSResponse = sonic_rs::from_str(&body).unwrap();
    let mut filtered_data = String::new();
    for answer in p.answer {
        if let Some(content) = answer.data.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            // Проверяем формат "N:base64data"
            if let Some((prefix, data)) = content.split_once(':') {
                if prefix.chars().all(|c| c.is_ascii_digit()) {
                    filtered_data.push_str(data);
                    continue;
                }
            }
        }
    }
    Ok(filtered_data)
}