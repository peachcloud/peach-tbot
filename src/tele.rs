use telegram_bot::*;
use telegram_bot::{Api, Message, SendMessage, MessageKind, UpdateKind};
use std::result::Result;
use futures::StreamExt;
extern crate pnet;
use pnet::datalink;
use pnet::ipnetwork;
use std::process::Command;


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

    pub async fn poll(&self) -> Result<(), telegram_bot::Error>{
        // Fetch new updates via long poll method
        let mut stream = self.api.stream();
        while let Some(update) = stream.next().await {
            // If the received update contains a new message...
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    // Print received text message to stdout.
                    println!("<{}>: {}", &message.from.first_name, data);
                    println!("user id: {}", &message.from.id);
                    let command = data.as_str();
                    if command == "ip" {
                        self.log_ip().await?;
                    } else if command == "public" {
                        self.log("++ attempting to get public ip").await?;
                        let public_ip =  self.public_ip().await;
                        self.log(&format!("++ public ip: {}", public_ip)).await?
                    }
                    else {
                        // unknown command
                        self.api.send(message.text_reply(format!(
                            "unknown command '{}'", data
                        )))
                        .await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn log_ip(&self) -> Result<(), telegram_bot::Error> {
        let ifaces= ["eth0", "wlan0"];
        self.log("++ attemping to get ip address").await?;
        for iface in datalink::interfaces() {
            if ifaces.contains(&iface.name.as_str()) {
                println!("{}: {:?}", &iface.name, iface);
                // loop through ips
                for ip in iface.ips {
                    match ip {
                        ipnetwork::IpNetwork::V4(ipNetwork) => {
                            println!("ip: {:?}", ipNetwork.ip());
                            let log_msg = &format!("++ {} ip: {}", &iface.name, ipNetwork.ip());
                            println!("{}", log_msg);
                            self.log(log_msg).await?;
                        }
                        _ => {
                            println!("no match");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn public_ip(&self) -> String {
        let output = Command::new("/usr/bin/curl")
            .arg("ifconfig.me")
            .output().expect("failed to get public IP");
        let command_output = std::str::from_utf8(&output.stdout).expect("Incorrect format");
        command_output.to_string()
    }
}