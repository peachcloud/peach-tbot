use std::env;
use dotenv;

use futures::StreamExt;
use telegram_bot::*;
use telegram_bot::{Api, Message, SendMessage, MessageKind, UpdateKind};
mod email;
mod tele;
use tele::TStruct;
use std::result::Result;
use std::{thread, time};



#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

//      api.send(message.text_reply(format!("+ wgbot is booting up")));

    let imap_username = env::var("IMAP_USERNAME").expect("imap username is not set");
    let imap_password = env::var("IMAP_PASSWORD").expect("imap password is not set");
    let telegram_log_id: i64 = env::var("TELEGRAM_LOG_ID").expect("TELEGRAM_LOG_ID is not set").parse().unwrap();

    let t:  TStruct = TStruct::new(api, telegram_log_id);
    t.test();
    t.log("++ wgbot is online").await?;

    loop {
       println!("++ fetching new emails");
       let email = email::fetch_inbox_top(&imap_username, &imap_password, &t);
        match email {
            Ok(v) => {
                println!("positive result");
                match v {
                    Some(x) => {
                        let log_msg = format!("++ new listing: {}", x);
                        t.log(&log_msg).await?;
                    }
                    None => println!("None returned")
                }
            },
            Err(e) => println!("error: {:?}", e),
        }
        let wait_time = time::Duration::from_millis(5000);
        thread::sleep(wait_time);
    }

//
//    // Fetch new updates via long poll method
//    let mut stream = api.stream();
//    while let Some(update) = stream.next().await {
//        // If the received update contains a new message...
//        let update = update?;
//        if let UpdateKind::Message(message) = update.kind {
//            if let MessageKind::Text { ref data, .. } = message.kind {
//                // Print received text message to stdout.
//                println!("<{}>: {}", &message.from.first_name, data);
//                println!("user id: {}", &message.from.id);
//
//                // Answer message with "Hi".
//                api.send(message.text_reply(format!(
//                    "Hi, {}! You just wrote '{}'",
//                    &message.from.first_name, data
//                )))
//                .await?;
//            }
//        }
//    }
    Ok(())
}