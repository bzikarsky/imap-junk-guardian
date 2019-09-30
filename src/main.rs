extern crate email;
extern crate imap;
extern crate native_tls;

use crate::config::Config;
use crate::imap_wrapper::{Result, MailboxSession, Mail};

mod config;
mod util;
mod imap_wrapper;

fn main() -> Result<()> {
    let cfg = Config::from_env();
    let mut session = MailboxSession::connect(&cfg)?;

    loop {
        let mails = session.unseen_mails()?;

        if mails.is_empty() {
            println!("{} does not contain unseen messages, nothing is moved", cfg.mailbox);
        } else {
            println!("Unseen mails:");
            mails.iter().for_each(|Mail {uid, subject}| println!("  {}: {}", uid, subject));

            match session.mv(&mails, &cfg.destination_mailbox) {
                Ok(_) => println!("Moved {} unseen messages from {} to {}", mails.len(), cfg.mailbox, cfg.destination_mailbox),
                Err(e) => println!("Something went wrong: {}", e.to_string())
            }
        }

        println!("Will IDLE and wait for changes in {} now", &cfg.mailbox);
        session.idle_and_keepalive()?;
    }
}