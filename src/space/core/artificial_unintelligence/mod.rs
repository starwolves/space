use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

use crate::space::StartupLabels;

use self::{
    resources::ContextMapVectors,
    systems::{find_path::find_path, spawn_ai::spawn_ai, steer::steer},
};

pub mod components;
pub mod functions;
mod resources;
pub mod systems;

pub struct ArtificialUnintelligencePlugin;

impl Plugin for ArtificialUnintelligencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContextMapVectors>()
            .add_startup_system(spawn_ai.after(StartupLabels::InitDefaultGridmapData))
            .add_system(find_path)
            .add_system(steer);
    }
}
