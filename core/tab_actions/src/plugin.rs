use bevy::prelude::{App, Plugin, ResMut, SystemSet};
use shared::tab_actions::QueuedTabActions;

use super::tab_data::tab_data;
use bevy::app::CoreStage::PostUpdate;

pub struct TabActionsPlugin;

impl Plugin for TabActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tab_data)
            .init_resource::<QueuedTabActions>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new().with_system(clear_tab_actions_queue),
            );
    }
}

pub fn clear_tab_actions_queue(mut queue: ResMut<QueuedTabActions>) {
    queue.queue.clear();
}
