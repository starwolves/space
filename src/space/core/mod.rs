use bevy_log::info;

pub mod asana;
pub mod atmospherics;
pub mod chat;
pub mod combat;
pub mod configuration;
pub mod connected_player;
pub mod console_commands;
pub mod data_link;
pub mod entity;
pub mod examinable;
pub mod gridmap;
pub mod health;
pub mod humanoid;
pub mod inventory;
pub mod inventory_item;
pub mod map;
pub mod networking;
pub mod pawn;
pub mod physics;
pub mod rigid_body;
pub mod sensable;
pub mod senser;
pub mod sfx;
pub mod static_body;
pub mod tab_actions;
pub mod world_environment;

pub fn server_is_live() {
    info!("Live.");
}
