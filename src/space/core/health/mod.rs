use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::SystemSet;

use crate::space::PostUpdateLabels;

use self::{
    entity_update::health_ui_update,
    events::{Attack, NetHealthUpdate},
    resources::ClientHealthUICache,
};

pub mod components;
pub mod entity_update;
pub mod events;
pub mod resources;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientHealthUICache>()
            .add_event::<NetHealthUpdate>()
            .add_event::<Attack>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(health_ui_update),
            );
    }
}
