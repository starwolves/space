use bevy::prelude::info;

pub mod pawn;
pub mod inventory;
pub mod inventory_item;
pub mod sfx;
pub mod health;
pub mod rigid_body;
pub mod static_body;
pub mod physics;
pub mod gridmap;
pub mod entity;
pub mod world_environment;
pub mod configuration;
pub mod networking;
pub mod atmospherics;
pub mod data_link;
pub mod map;

pub fn server_is_live() {
    info!("Live.");
}
