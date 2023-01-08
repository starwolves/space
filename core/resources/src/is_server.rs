use std::env;

pub fn is_server() -> bool {
    match env::args().nth(1) {
        Some(c) => {
            if c == "server" {
                true
            } else {
                false
            }
        }
        None => false,
    }
}
