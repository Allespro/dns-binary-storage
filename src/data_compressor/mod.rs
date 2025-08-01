use std::error::Error;
use std::io::{Read, Write};
use zstd::stream::read::Decoder;
use zstd::stream::write::Encoder;
use base64ct::{Base64, Encoding};

#[inline]
fn decompress_bytes_zstd(input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decoder = Decoder::new(input)?;
    let mut output = Vec::new();
    decoder.read_to_end(&mut output)?;
    Ok(output)
}

#[inline]
fn compress_bytes_zstd(input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut encoder = Encoder::new(Vec::new(), 22)?; // Понижаем уровень сжатия с 22 до 3
    encoder.write_all(input)?;
    encoder.finish().map_err(Into::into)
}

pub fn bytes_to_dns(input: &[u8], domain: &str, ttl: u32, chunk_size: usize) -> Result<String, Box<dyn Error>> {
    let compressed = compress_bytes_zstd(input)?;
    let encoded= Base64::encode_string(&compressed);

    let mut result = String::with_capacity(encoded.len() * 2);

    for (i, chunk) in encoded.as_bytes().chunks(chunk_size).enumerate() {
        if i > 0 {
            result.push('\n');
        }
        result.push_str(domain);
        result.push_str(&format!(".\t{ttl}\tIN\tTXT\t\"{i:x}:"));
        result.push_str(unsafe { std::str::from_utf8_unchecked(chunk) });
        result.push('"');
    }

    Ok(result)
}

#[inline]
pub fn base64_to_bytes(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = Base64::decode_vec(&input).unwrap();
    decompress_bytes_zstd(&bytes)
}