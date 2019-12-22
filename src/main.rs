mod alla;
mod bis;

use std::env;

use regex::Regex;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use alla::Alla;
use bis::Bis;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let spaces: Regex = Regex::new(" +").unwrap();
        let msg_parts: Vec<&str> = spaces.split(msg.content.as_str()).collect();

        let response = match msg_parts[0] {
            "!alla" => Some(Alla::accept_raw(msg_parts[1..].to_vec())),
            "!bis" => Some(Bis::accept_raw(msg_parts[1..].to_vec())),
            _ => None,
        };

        if let Some(to_send) = response {
            if let Err(why) = msg.channel_id.say(&ctx.http, to_send) {
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
