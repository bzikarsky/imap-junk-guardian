use std::env;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub mailbox: String,
    pub destination_mailbox: String
}


impl Config {
    fn new() -> Self {
        Config {
            host: "outlook.office365.com".to_string(),
            port: 993,
            user: "foo".to_string(),
            pass: "bar".to_string(),
            mailbox: "Junk".to_string(),
            destination_mailbox: "INBOX".to_string()
        }
    }

    pub fn from_env() -> Self {
        let mut cfg = Config::new();

        if let Ok(user) = env::var("IMAP_USER") {
            cfg.user = user
        }

        if let Ok(pass) = env::var("IMAP_PASSWORD") {
            cfg.pass = pass
        }

        println!("{:?}", cfg);

        return cfg
    }
}