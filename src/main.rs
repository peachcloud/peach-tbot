use std::env;
use dotenv;

use futures::StreamExt;
use telegram_bot::*;
use telegram_bot::{Api, Message, SendMessage, MessageKind, UpdateKind};
mod tele;
use tele::TStruct;
use std::result::Result;
use std::{thread, time};







#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::from_path("/etc/peachtbot.conf").ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let telegram_log_id: i64 = env::var("TELEGRAM_LOG_ID").expect("TELEGRAM_LOG_ID is not set").parse().unwrap();

    let t:  TStruct = TStruct::new(api, telegram_log_id);
    t.test();
    t.log("++ peach-tbot is online").await?;

    t.log_ip().await?;
    t.poll().await?;

    Ok(())
}