#[macro_use]
extern crate clap;
use clap::App;
extern crate serde;
extern crate reqwest;
extern crate colored;
extern crate chrono;

use std::error::Error;
use std::cmp::{max, min};
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde::{Serialize, Deserialize};
use colored::*;
use chrono::prelude::*;

fn main() -> () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let auth: &str = matches.value_of("authorization").unwrap();
    let display_name: &str = matches.value_of("name").unwrap();
    get_messages(auth, display_name);
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    composetime: DateTime<Local>,
    content: String,
    messagetype: String,
    imdisplayname: String,
}

fn get_messages(auth: &str, disaply_name: &str) -> Result<(), Box<Error>> {
    let colors = vec![
        31,  // Red
        32,  // Green
        33,  // Yellow
        34,  // Blue
        35,  // Purple
        36]; // Cyan ;

    let url = "https://bl2pv2.ng.msg.teams.microsoft.com/v1/users/ME/conversations/19%3Aa1140af8b07f4bfca44a02cd9aee0248%40thread.skype/messages?view=msnp24Equivalent|supportsMessageProperties&pageSize=200&startTime=1";
    let mut headers = HeaderMap::new();
    headers.insert("Authentication", HeaderValue::from_str(auth)?);

    let client = Client::new();
    let mut res = client.get(url).headers(headers).send()?;
    if res.status().is_success() {
        let response: Response = res.json()?;
        let mut messages = response.messages;
        // TODO: move to own function
        messages.sort_by(|a, b| a.composetime.cmp(&b.composetime));
        let mut previous_date: DateTime<Local> = messages[0].composetime;
        let mut color = colors[max(min(colors.len(), messages[0].composetime.day() as usize), 0)];
        for mut message in messages {
            match message.messagetype.as_ref() {
                "Text" => {
                    if message.imdisplayname.eq(disaply_name) {
                        if !previous_date.day().eq(&message.composetime.day()) {
                            color = colors[max(min(colors.len(), message.composetime.day() as usize), 0)];
                            println!("\n\x1B[{}m{}\x1B[0m ", color, message.composetime.format("%a %b %e %T %Y").to_string().bold().underline());
                            previous_date = message.composetime;
                        }
                        if message.content.len() > 103 { // TODO: check for last char being whitespace
                            message.content.truncate(100);
                            message.content.push_str("...");
                        }
                        println!("\x1B[{}m{}\x1B[0m {}", color, message.composetime.format("%Y-%m-%d %H:%M:%S").to_string(), message.content);
                    }
                }
                _ => {}
            }
        }
    } else {
        // TODO: handle 401 unauthorized
        println!("{:#?}", res.status());
    }


    Ok(())
}


