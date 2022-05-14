use bevy_app::{App, Plugin};
use bevy_ecs::schedule::SystemSet;

pub mod components;
pub mod entity_update;
pub mod process_content;
pub mod spawn;

use bevy_app::CoreStage::PostUpdate;

use crate::core::PostUpdateLabels;

use self::entity_update::omni_light_update;

pub struct OmniLightPlugin;

impl Plugin for OmniLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(omni_light_update),
        );
    }
}
