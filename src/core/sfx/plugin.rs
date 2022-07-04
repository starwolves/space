use bevy::{
    core::FixedTimestep,
    prelude::{App, Plugin, SystemSet},
};

use crate::core::{
    entity::entity_data::INTERPOLATION_LABEL1, space_plugin::plugin::PostUpdateLabels,
};

use super::{
    entity_update::{repeating_sfx_update, sfx_update},
    timers::{free_sfx, tick_timers_slowed, SfxAutoDestroyTimers},
};
use bevy::app::CoreStage::PostUpdate;

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
            )
            .add_system(free_sfx);
    }
}
