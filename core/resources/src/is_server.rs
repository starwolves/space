use std::env;

use bevy::prelude::warn;

pub fn is_server() -> bool {
    match env::var("CARGO_MANIFEST_DIR") {
        Ok(r) => r.ends_with("server"),
        Err(_) => {
            warn!("CARGO_MANIFEST_DIR not set, not running server.");
            false
        }
    }
}
