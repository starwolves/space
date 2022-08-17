use api::{actions::QueuedActions, data::ActionsLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};

use crate::{
    action::{clear_action_building, clear_actions_queue, init_action_building},
    data::{
        action_data_build_interacted_entity, action_data_finalizer, init_action_data_building,
        ActionDataRequests, ActionIncremented, ActionRequests, BuildingActions,
    },
};

use bevy::app::CoreStage::PostUpdate;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_action_data_building.label(ActionsLabels::Init))
            .add_system(
                action_data_build_interacted_entity
                    .after(ActionsLabels::Init)
                    .label(ActionsLabels::Build),
            )
            .add_system(action_data_finalizer.after(ActionsLabels::Approve))
            .init_resource::<QueuedActions>()
            .init_resource::<BuildingActions>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new().with_system(clear_actions_queue),
            )
            .init_resource::<ActionIncremented>()
            .init_resource::<ActionDataRequests>()
            .add_system(init_action_building.label(ActionsLabels::Init))
            .add_system(
                clear_action_building
                    .label(ActionsLabels::Clear)
                    .before(ActionsLabels::Init),
            )
            .init_resource::<ActionRequests>();
    }
}
