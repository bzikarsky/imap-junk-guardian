#![allow(dead_code)]
use std::net::TcpStream;

use imap::types::{Flag, Uid};
use native_tls::TlsStream;
use log::info;

use crate::util::RfC2047EncodedStr;
use crate::config::Config;

type Session = imap::Session<TlsStream<TcpStream>>;
pub type Result<T> = imap::error::Result<T>;

pub struct MailboxSession {
    session: Session,
    mailbox: String
}

impl MailboxSession {
    pub fn connect(cfg: &Config) -> Result<Self> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();

        info!("Connecting to {}:{}", cfg.host, cfg.port);
        let client = imap::connect((cfg.host.as_str(), cfg.port), &cfg.host, &tls)?;

        info!("Logging in as {}", cfg.user);
        let session = client.login(&cfg.user, &cfg.pass)
            .map_err(|e| e.0)?;

        MailboxSession::new(session, &cfg.mailbox)
    }

    fn new(mut session: Session, mailbox: &str) -> Result<Self> {
        info!("Selecting mailbox {}", mailbox);
        session.select(mailbox)?;

        Ok(MailboxSession {
            session,
            mailbox: mailbox.to_string()
        })
    }

    pub fn switch(self, mailbox: &str) -> Result<Self> {
        MailboxSession::new(self.session, mailbox)
    }


    pub fn unseen_mails(&mut self) -> Vec<Mail> {
        match self.session.fetch("1:*", "(ENVELOPE UID FLAGS)") {
            Ok(fetch) => fetch.iter().filter_map(|mail| {
                let subject = mail.envelope()?.subject.unwrap().rfc2047_decode();
                let uid = mail.uid.unwrap();
                let seen = mail.flags().iter().any(|f| match f {
                    Flag::Seen => true,
                    _ => false
                });

                if !seen {
                    Some(Mail { uid, subject })
                } else {
                    None
                }
            }).collect(),
            Err(_) => Vec::new()
        }

    }

    pub fn mv(&mut self, uids: &Vec<Mail>, destination: &str) -> Result<()> {
        info!("Moving {} mails to {}", uids.len(), destination);
        let string_uids: Vec<String> = uids.iter().map(|mail| mail.uid.to_string()).collect();
        self.session.uid_mv(string_uids.join(","), destination)
    }

    pub fn idle_and_keepalive(&mut self) -> Result<()> {
        self.session.idle()?.wait_keepalive()
    }
}

pub struct Mail {
    pub uid: Uid,
    pub subject: String
}