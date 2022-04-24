pub mod components;
pub mod systems;
use bevy_app::{App, CoreStage::PostUpdate, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

use crate::space::PostUpdateLabels;

use self::systems::visible_checker::visible_checker;

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
