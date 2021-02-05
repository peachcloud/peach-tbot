extern crate imap;
extern crate native_tls;
use telegram_bot::{Api, Error};
use crate::tele::TStruct;
use std::result::Result;
use imap::types::{Fetch, Seq};
use imap::Session;


//fn delete(seq: imap::types::Seq, s: &mut Session<TcpStream>) -> imap::error::Result<()> {
//    s.store(format!("{}", seq), "+FLAGS (\\Deleted)")?;
//    s.expunge()?;
//    Ok(())
//}


pub fn fetch_inbox_top(imap_username: &str, imap_password: &str, telegram: &TStruct) -> imap::error::Result<Option<String>> {
    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(imap_username, imap_password)
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("wgbot")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;

    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    println!("{}", body);
    let mut split = body.lines();
    let split_vec: Vec<&str> = split.collect();
    let mut to_return : Option<String> = None;
    for line in split_vec {
        if line.contains("suchauftrag_detail") {
            to_return = Some(String::from(line.trim_start()))
        }
    }

    // now delete the email
    let seq : Seq = message.message;
    imap_session.store(format!("{}", seq), "+FLAGS (\\Deleted)")?;
    imap_session.expunge()?;


//    let result : Result<(), Error> = telegram.log("test message 3").await;
//    match result {
//        Ok(v) => println!("++ logged"),
//        Err(e) => println!("++ error logging: {?}", e)
//    }

    // be nice to the server and log out
    imap_session.logout()?;

//    Ok(Some(body))
    Ok(to_return)
}