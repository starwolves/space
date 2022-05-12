pub mod components;
pub mod entity_update;
pub mod functions;
pub mod systems;

pub struct PhysicsPlugin;

use bevy::prelude::SystemSet;
use bevy_app::{App, CoreStage::PostUpdate, Plugin};

use self::{entity_update::world_mode_update, systems::physics_events};

use super::plugin::PostUpdateLabels;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(physics_events).add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(world_mode_update),
        );
    }
}
