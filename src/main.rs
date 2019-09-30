extern crate email;
extern crate imap;
extern crate native_tls;

use std::net::TcpStream;

use imap::types::{Flag, Uid};
use native_tls::TlsStream;

use config::Config;

mod config;
mod util;

type Session = imap::Session<TlsStream<TcpStream>>;
type Result<T> = imap::error::Result<T>;

fn main() -> Result<()> {
    let cfg = Config::from_env();
    let mut session = connect(&cfg)?;

    session.select(&cfg.source_mailbox)?;

    loop {
        let uids: Vec<String> = find_unseen_mails(&mut session)?.iter().map(|uid| uid.to_string()).collect();

        if uids.is_empty() {
            println!("{} does not contain unseen messages, nothing is moved", cfg.source_mailbox);
        } else {
            session.uid_mv(uids.join(","), &cfg.destination_mailbox)?;
            println!("Moved {} unseen messages from {} to {}", uids.len(), cfg.source_mailbox, cfg.destination_mailbox)
        }

        println!("Will IDLE and wait for changes in {} now", &cfg.source_mailbox);
        session.idle()?.wait_keepalive()?;
    }
}


fn connect(cfg: &Config) -> Result<Session> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((cfg.host.as_str(), cfg.port), &cfg.host, &tls)?;

    client.login(&cfg.user, &cfg.pass)
        .map_err(|e| e.0)
}

fn find_unseen_mails(session: &mut Session) -> Result<Vec<Uid>> {
    println!("Unseen mails:");

    let uids: Vec<Uid> = session.fetch("1:*", "(ENVELOPE UID FLAGS)")?.iter().filter_map(|mail| {
        let subject = mail.envelope()?.subject.unwrap().rfc2047_decode();
        let uid = mail.uid.unwrap();
        let seen = mail.flags().iter().any(|f| match f {
            Flag::Seen => true,
            _ => false
        });


        if !seen {
            println!("  {}: {}", uid, subject);
            Some(uid)
        } else {
            None
        }
    }).collect();

    Ok(uids)
}