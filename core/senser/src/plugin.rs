use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use shared::data::PostUpdateLabels;

use super::visible_checker::visible_checker;
use bevy::app::CoreStage::PostUpdate;

pub struct SenserPlugin;

impl Plugin for SenserPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            PostUpdate,
            visible_checker
                .after(PostUpdateLabels::SendEntityUpdates)
                .label(PostUpdateLabels::VisibleChecker),
        );
    }
}
