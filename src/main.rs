use std::env;

use regex::Regex;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest::Response;
use select::{document::Document, predicate::Name};

struct Handler;

impl Handler {
    fn do_search(&self, item_name: &str) -> String {
        let encoded_item = percent_encode(item_name.as_bytes(), NON_ALPHANUMERIC);
        let url = format!(
            "http://everquest.allakhazam.com/search.html?q={}",
            encoded_item
        );

        //TODO: lol error checking
        let response = reqwest::get(url.as_str()).unwrap();
        assert!(response.status().is_success());

        let links = Handler::get_link_name_pairs(response);
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
        //TODO: lol error checking
        let response = reqwest::get(link).unwrap();
        assert!(response.status().is_success());

        let document = Document::from_read(response).unwrap();

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

    fn get_link_name_pairs(response: Response) -> Vec<(String, String)> {
        let document = Document::from_read(response).unwrap();

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

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let spaces: Regex = Regex::new(" +").unwrap();

        if msg.content.starts_with("!alla ") {
            // Split content
            let msg_parts: Vec<&str> = spaces.splitn(msg.content.as_str(), 2).collect();

            if msg_parts.len() == 1 {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Usage: '!itemsearch <item>'") {
                    println!("Error sending message: {:?}", why);
                }

                return;
            }

            let response = self.do_search(msg_parts[1]);
            if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
