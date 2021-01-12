extern crate email;
extern crate imap;
extern crate native_tls;
extern crate log;
extern crate pretty_env_logger;

use log::{info, error, warn};

use crate::config::Config;
use crate::imap_wrapper::{Result, MailboxSession, Mail};

mod config;
mod util;
mod imap_wrapper;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let cfg = Config::from_env();

    let mut session = connect(&cfg);

    loop {
        let mails = session.unseen_mails();

        if mails.is_empty() {
            info!("{} does not contain unseen messages, nothing is moved", cfg.mailbox);
        } else {
            info!("Unseen mails:");
            mails.iter().for_each(|Mail {uid, subject}| info!("  {}: {}", uid, subject));

            match session.mv(&mails, &cfg.destination_mailbox) {
                Ok(_) => info!("Moved {} unseen messages from {} to {}", mails.len(), cfg.mailbox, cfg.destination_mailbox),
                Err(e) => error!("Something went wrong: {}", e.to_string())
            }
        }

        info!("Will IDLE and wait for changes in {} now", &cfg.mailbox);

        if let Err(e) = session.idle_and_keepalive() {
            warn!("Session idle and keepalive failed: {}", e);
            warn!("Reconnecting...");
            session = connect(&cfg);
            continue
        }

        info!("{} changed - will check again", &cfg.mailbox)
    }
}

fn connect(cfg: &Config) -> MailboxSession {
    match MailboxSession::connect(&cfg) {
        Ok(session) => session,
        Err(e) => panic!("Connect and select failed: {}", e)
    }
}