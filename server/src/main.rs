pub mod server;

use bevy::prelude::info;

use crate::server::start_server;

fn main() {
    start_server();
}
pub fn server_is_live() {
    info!("Live.");
}
