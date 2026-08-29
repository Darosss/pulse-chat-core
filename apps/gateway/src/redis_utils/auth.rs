pub fn get_blacklist_key(jti: &str) -> String {
    format!("auth:blacklist:jti:{}", jti)
}
