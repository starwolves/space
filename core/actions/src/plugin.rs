use std::env;

use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::server::net_system;
use resources::labels::{ActionsLabels, PostUpdateLabels, PreUpdateLabels};

use crate::{
    core::{
        clear_action_building, init_action_data_listing, init_action_request_building,
        list_action_data_finalizer, list_action_data_from_actions_component, ActionIncremented,
        ActionRequests, BuildingActions, InputAction, InputListActionsEntity, InputListActionsMap,
        ListActionDataRequests, NetActionDataFinalizer,
    },
    networking::incoming_messages,
};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(init_action_data_listing.label(ActionsLabels::Init))
                .add_system(
                    list_action_data_from_actions_component
                        .after(ActionsLabels::Init)
                        .label(ActionsLabels::Build),
                )
                .add_system(list_action_data_finalizer.after(ActionsLabels::Approve))
                .init_resource::<BuildingActions>()
                .init_resource::<ActionIncremented>()
                .init_resource::<ListActionDataRequests>()
                .add_system(init_action_request_building.label(ActionsLabels::Init))
                .add_system(
                    clear_action_building
                        .label(ActionsLabels::Clear)
                        .before(ActionsLabels::Init),
                )
                .add_event::<NetActionDataFinalizer>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetActionDataFinalizer>),
                )
                .init_resource::<ActionRequests>()
                .add_system_to_stage(
                    PreUpdate,
                    incoming_messages
                        .after(PreUpdateLabels::NetEvents)
                        .label(PreUpdateLabels::ProcessInput),
                )
                .add_event::<InputListActionsMap>()
                .add_event::<InputListActionsEntity>()
                .add_event::<InputAction>();
        }
    }
}
