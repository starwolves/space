use bevy::prelude::{App, Plugin, SystemSet};

use crate::core::space_plugin::plugin::PostUpdateLabels;

use super::{entity_update::world_mode_update, physics_events::physics_events};
use bevy::app::CoreStage::PostUpdate;

pub struct PhysicsPlugin;

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
