use bevy_app::{App, Plugin};

pub mod components;
pub mod entity_update;
pub mod process_content;
pub mod spawn;
use bevy_app::CoreStage::PostUpdate;
use bevy_ecs::schedule::SystemSet;

use crate::core::space_plugin::PostUpdateLabels;

use self::entity_update::gi_probe_update;

pub struct GIProbePlugin;

impl Plugin for GIProbePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(gi_probe_update),
        );
    }
}
