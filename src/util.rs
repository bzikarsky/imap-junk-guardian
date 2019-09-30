pub trait RfC2047EncodedStr {
    fn rfc2047_decode(&self) -> String;
}

impl RfC2047EncodedStr for str {
    fn rfc2047_decode(&self) -> String {
        if self.starts_with("=?") {
            self
                .split(" ")
                .map(|w| {
                    email::rfc2047::decode_rfc2047(w)
                        .unwrap_or("<decode failed>".to_string())
                        .replace('_', " ")
                })
                .collect::<Vec<String>>()
                .join("")
        } else {
            self.to_string()
        }
    }
}
