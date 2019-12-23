use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use crate::util::AllaUtil;
use select::document::Document;
use select::predicate::Name;

pub struct Alla;

impl Alla {
    pub fn accept_raw(msg_parts: Vec<&str>) -> String {
        let item_name: String = msg_parts.join(" ");
        Alla::do_search(item_name.as_str())
    }

    fn do_search(item_name: &str) -> String {
        let encoded_item = percent_encode(item_name.as_bytes(), NON_ALPHANUMERIC);
        let url = format!(
            "http://everquest.allakhazam.com/search.html?q={}",
            encoded_item
        );

        let document = match AllaUtil::fetch_url(url.as_str()) {
            Ok(d) => d,
            Err(e) => return e,
        };

        let links = Alla::get_link_name_pairs(document);
        if links.len() == 0 {
            return format!("No results found for \"{}\"", item_name);
        } else if links.len() > 3 {
            return format!("Too many results for \"{}\"", item_name);
        } else {
            let details = Alla::get_details(links);
            return Alla::format_response(details);
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
                let detail = Alla::get_detail(&link);
                (link, String::from(name), detail)
            })
            .collect();
    }

    fn get_detail(link: &String) -> String {
        let document = match AllaUtil::fetch_url(link) {
            Ok(d) => d,
            Err(e) => return e,
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
