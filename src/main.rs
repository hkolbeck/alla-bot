use std::collections::HashSet;
use std::env;

use regex::Regex;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use select::{document::Document, predicate::Name};

use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        let document = Document::from_read(response).unwrap();

        let links: HashSet<(&str, String)> = document
            .find(Name("a"))
            .filter_map(|n| n.attr("href").map(|link| (link, n.text())))
            .filter(|(link, _)| link.starts_with("/db/item.html?item="))
            .filter(|(_, name)| name.len() > 0)
            .collect();

        self.format_response(item_name, links)
    }

    fn format_response(&self, search: &str, links: HashSet<(&str, String)>) -> String {
        if links.len() == 0 {
            return format!("No results found for \"{}\"", search);
        } else if links.len() <= 3 {
            let mut result = String::new();
            links.iter().for_each(|(link, name)| {
                result.push_str(
                    format!("{} - http://everquest.allakhazam.com{}\n", name, link).as_str(),
                )
            });
            return result;
        } else {
            return format!("Too many results for \"{}\"", search);
        }
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let spaces: Regex = Regex::new(" +").unwrap();

        if msg.content.starts_with("!is ") {
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
