use bevy_app::{App, Plugin};
use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemLabel, SystemSet},
    system::ResMut,
};

use self::{
    events::InputTabAction,
    resources::QueuedTabActions,
    systems::{tab_action::tab_action, tab_data::tab_data},
};

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;
use bevy_app::CoreStage::PostUpdate;

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
