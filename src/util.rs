use select::document::Document;
use std::borrow::Borrow;
use std::io::Read;

pub struct AllaUtil;

impl AllaUtil {
    pub fn fetch_url(url: &str) -> Result<Document, String> {
        let response = match reqwest::get(url) {
            Ok(x) => x,
            Err(e) => return Err(format!("Error issuing request: {}", e)),
        };

        if !response.status().is_success() {
            return Err(format!("Request failed: {}", response.status().as_str()));
        }

        let bytes: Vec<u8> = response
            .bytes()
            .take_while(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        let decoded = String::from_utf8_lossy(bytes.as_slice());

        Ok(Document::from(decoded.borrow()))
    }
}
