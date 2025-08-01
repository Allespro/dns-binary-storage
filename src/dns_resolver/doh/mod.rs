use minreq;
use sonic_rs::{Deserialize};

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
    let binding = minreq::get(resolver)
        .with_param("name", domain)
        .with_param("type", "TXT")
        .with_header("accept", "application/dns-json")
        .send()?;

    let p: DNSResponse = sonic_rs::from_str(binding.as_str()?).unwrap();

    // pfix, daata
    let mut records: Vec<(u16, String)> = Vec::new();

    for answer in p.answer {
        if let Some(content) = answer.data.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            // format should be "N:base64data"
            if let Some((prefix, data)) = content.split_once(':') {
                if prefix.chars().all(|c| c.is_ascii_digit()) {
                    if let Ok(prefix_num) = prefix.parse::<u16>() {
                        records.push((prefix_num, data.to_string()));
                    }
                }
            }
        }
    }

    // sort by 16bit pfix
    records.sort_by_key(|&(prefix, _)| prefix);

    let mut filtered_data = String::new();
    for (_, data) in records {
        filtered_data.push_str(&data);
    }

    Ok(filtered_data)
}