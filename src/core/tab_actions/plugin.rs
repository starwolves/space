use bevy::prelude::{
    App, ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemLabel, SystemSet,
};

use super::{
    tab_action::{tab_action, InputTabAction, QueuedTabActions},
    tab_data::tab_data,
};
use bevy::app::CoreStage::PostUpdate;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum TabActionsQueueLabels {
    TabAction,
}

pub struct TabActionsPlugin;

impl Plugin for TabActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tab_data)
            .add_system(tab_action.label(TabActionsQueueLabels::TabAction))
            .add_event::<InputTabAction>()
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
