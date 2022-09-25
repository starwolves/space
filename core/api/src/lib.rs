//! Where shared code is put, useful in order to avoid cyclic dependency issues.
//! However, this crate can and must be reduced a lot.

pub mod chat;
pub mod combat;
pub mod connected_player;
pub mod console_commands;
pub mod converters;
pub mod data;
pub mod data_link;
pub mod entity_updates;
pub mod examinable;
pub mod get_spawn_position;
pub mod gridmap;
pub mod health;
pub mod humanoid;
pub mod inventory;
pub mod load_entity;
pub mod network;
pub mod pawn;
pub mod rigid_body;
pub mod sensable;
pub mod senser;
pub mod ui;
pub mod world_environment;
