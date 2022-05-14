pub mod components;
pub mod entity_update;
pub mod functions;
pub mod systems;

pub struct PhysicsPlugin;

use bevy_app::{App, CoreStage::PostUpdate, Plugin};
use bevy_ecs::schedule::SystemSet;

use self::{entity_update::world_mode_update, systems::physics_events};

use super::PostUpdateLabels;

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
