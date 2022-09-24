//! Server launcher and loop initializer.

use crate::server::configure_and_start_server;
use bevy::prelude::info;

/// Loop builder.
mod server;

/// The function that launches the server on application start.
fn main() {
    configure_and_start_server();
}

/// Prints "Live." from main module for fancy text output.
fn server_is_live() {
    info!("Live.");
}
