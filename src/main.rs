pub mod core;
pub mod entities;
pub mod server;

use bevy::prelude::info;
use server::server;

fn main() {
    server();
}
pub fn server_is_live() {
    info!("Live.");
}
