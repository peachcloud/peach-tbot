use telegram_bot::*;
use telegram_bot::{Api, Message, SendMessage, MessageKind, UpdateKind};
use std::result::Result;


pub struct TStruct {
    api: Api,
    log_id: i64,
    chat_id: ChatId
}

impl TStruct {
    pub fn new(api: Api, log_id: i64) -> Self {
        Self {
            api,
            log_id,
            chat_id: ChatId::new(log_id)
        }
    }
    pub fn test(&self) {
        println!("++ calling test function: {}", self.log_id);
    }
    pub async fn log(&self, msg: &str) -> Result<(), telegram_bot::Error> {
        println!("++ attempting to log {}", msg);
        let s:SendMessage = SendMessage::new(&self.chat_id, msg);
        self.api.send(s).await?;
        Ok(())
    }
}