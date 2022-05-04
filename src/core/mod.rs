use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;
use bevy_log::info;

use self::plugin::StartupLabels;

pub mod artificial_unintelligence;
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
pub mod plugin;
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

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            server_is_live
                .label(StartupLabels::ServerIsLive)
                .after(StartupLabels::ListenConnections),
        );
    }
}
