use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use select::document::Document;
use select::predicate::Name;

use reqwest::get;

pub struct Alla;

impl Alla {
    pub fn accept_raw(msg_parts: Vec<&str>) -> String {}

    fn do_search(item_name: &str) -> String {
        let encoded_item = percent_encode(item_name.as_bytes(), NON_ALPHANUMERIC);
        let url = format!(
            "http://everquest.allakhazam.com/search.html?q={}",
            encoded_item
        );

        let response = match reqwest::get(url.as_str()) {
            Ok(x) => x,
            Err(e) => return format!("Error issuing request: {}", e),
        };

        if !response.status().is_success() {
            if !response.status().is_success() {
                return format!("Request failed: {}", response.status().as_str(),);
            }
        }

        let document = match Document::from_read(response) {
            Ok(x) => x,
            Err(e) => return format!("Error reading response, try again later: {}", e),
        };

        let links = Handler::get_link_name_pairs(document);
        if links.len() == 0 {
            return format!("No results found for \"{}\"", item_name);
        } else if links.len() > 3 {
            return format!("Too many results for \"{}\"", item_name);
        } else {
            let details = Handler::get_details(links);
            return Handler::format_response(details);
        }
    }

    fn format_response(links: Vec<(String, String, String)>) -> String {
        let mut result = String::new();
        links.iter().for_each(|(link, name, detail)| {
            result.push_str(format!("{} - <{}>\n```\n{}\n```\n", name, link, detail).as_str())
        });

        result
    }

    fn get_details(links: Vec<(String, String)>) -> Vec<(String, String, String)> {
        return links
            .iter()
            .map(|(link, name)| (format!("http://everquest.allakhazam.com{}", link), name))
            .map(|(link, name)| {
                let detail = Handler::get_detail(&link);
                (link, String::from(name), detail)
            })
            .collect();
    }

    fn get_detail(link: &String) -> String {
        let response = match reqwest::get(link) {
            Ok(x) => x,
            Err(e) => return format!("Error issuing detail request: {}", e),
        };

        if !response.status().is_success() {
            return format!("Detail request failed: {}", response.status().as_str(),);
        }

        let document = match Document::from_read(response) {
            Ok(x) => x,
            Err(e) => return format!("Error reading detail response, try again later: {}", e),
        };

        let raw_detail: Vec<String> = document
            .find(Name("div"))
            .filter(|n| n.attr("class").eq(&Some("nobgrd")))
            .map(|n| n.text())
            .collect();

        if raw_detail.len() == 1 {
            return String::from(&raw_detail[0]);
        } else {
            panic!(raw_detail)
        }
    }

    fn get_link_name_pairs(document: Document) -> Vec<(String, String)> {
        return document
            .find(Name("a"))
            .filter_map(|n| {
                n.attr("href")
                    .map(|link| (String::from(link), String::from(n.text())))
            })
            .filter(|(link, _)| link.starts_with("/db/item.html?item="))
            .filter(|(_, name)| name.len() > 0)
            .map(|(link, name)| (String::from(link), String::from(name)))
            .collect();
    }
}
