impl str {
    pub fn rfc2047_decode(&self) -> String {
        if self.starts_with("=?") {
            raw
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
