use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Update};
use networking::messaging::{register_reliable_message, MessageSender};
use resources::{is_server::is_server, labels::ActionsLabels};

use crate::{
    core::{
        clear_action_building, init_action_data_listing, init_action_request_building,
        list_action_data_finalizer, list_action_data_from_actions_component, ActionIncremented,
        ActionRequests, BuildingActions, InputAction, InputListActions, ListActionDataRequests,
    },
    net::{ActionsClientMessage, ActionsServerMessage},
    networking::incoming_messages,
};
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                Update,
                (
                    init_action_data_listing.in_set(ActionsLabels::Init),
                    list_action_data_from_actions_component,
                    list_action_data_finalizer
                        .after(ActionsLabels::Approve)
                        .after(ActionsLabels::Init)
                        .in_set(ActionsLabels::Build),
                    init_action_request_building.in_set(ActionsLabels::Init),
                    clear_action_building
                        .in_set(ActionsLabels::Clear)
                        .before(ActionsLabels::Init),
                ),
            )
            .add_systems(PreUpdate, incoming_messages)
            .init_resource::<BuildingActions>()
            .init_resource::<ActionIncremented>()
            .init_resource::<ListActionDataRequests>()
            .init_resource::<ActionRequests>()
            .add_event::<InputListActions>()
            .add_event::<InputAction>();
        }

        register_reliable_message::<ActionsClientMessage>(app, MessageSender::Client);
        register_reliable_message::<ActionsServerMessage>(app, MessageSender::Server);
    }
}
