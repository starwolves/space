use bevy_app::{App, CoreStage::PostUpdate, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::schedule::SystemSet;

use self::{
    entity_update::{repeating_sfx_update, sfx_update},
    resources::SfxAutoDestroyTimers,
    systems::tick_timers_slowed,
};

use super::{entity::systems::broadcast_position_updates::INTERPOLATION_LABEL1, PostUpdateLabels};

pub mod components;
pub mod entity_update;
pub mod resources;
pub mod systems;

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SfxAutoDestroyTimers>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(tick_timers_slowed),
            )
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(sfx_update)
                    .with_system(repeating_sfx_update),
            );
    }
}
