use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use regex::Regex;

use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let spaces: Regex = Regex::new(" +").unwrap();

        if msg.content.starts_with("!is ") {
            // Split content
            let msg_parts: Vec<&str> = spaces.splitn(msg.content.as_str(), 2).collect();

            if msg_parts.len() == 1 {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Usage") {
                    println!("Error sending message: {:?}", why);
                }

                return;
            }

            let encoded_item = percent_encode(msg_parts[1].as_bytes(), NON_ALPHANUMERIC);
            let response = format!(
                "http://everquest.allakhazam.com/search.html?q={}",
                encoded_item
            );
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
