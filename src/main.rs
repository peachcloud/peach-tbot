use std::env;
use dotenv;

use futures::StreamExt;
use telegram_bot::*;
use telegram_bot::{Api, Message, SendMessage, MessageKind, UpdateKind};
mod email;


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

//      api.send(message.text_reply(format!("+ wgbot is booting up")));

    let imap_username = env::var("IMAP_USERNAME").expect("imap username is not set");
    let imap_password = env::var("IMAP_PASSWORD").expect("imap password is not set");
    let telegram_log_id: i64 = env::var("TELEGRAM_LOG_ID").expect("TELEGRAM_LOG_ID is not set").parse().unwrap();

    let chatId = ChatId::new(telegram_log_id);
    let s:SendMessage = SendMessage::new(chatId, "very cool that this is working");
    api.send(s).await?;

//    email::fetch_inbox_top(&imap_username, &imap_password);
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