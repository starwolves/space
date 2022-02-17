use bevy::prelude::info;

pub mod atmospherics;
pub mod combat;
pub mod configuration;
pub mod data_link;
pub mod entity;
pub mod gridmap;
pub mod health;
pub mod inventory;
pub mod inventory_item;
pub mod map;
pub mod networking;
pub mod pawn;
pub mod physics;
pub mod rigid_body;
pub mod sfx;
pub mod static_body;
pub mod world_environment;

pub fn server_is_live() {
    info!("Live.");
}
