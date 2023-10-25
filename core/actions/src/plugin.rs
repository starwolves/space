use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use networking::messaging::{register_reliable_message, MessageSender};
use resources::{
    modes::is_server_mode,
    sets::{ActionsSet, MainSet},
};

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
        if is_server_mode(app) {
            app.add_systems(
                FixedUpdate,
                (
                    init_action_data_listing.in_set(ActionsSet::Init),
                    list_action_data_from_actions_component
                        .after(ActionsSet::Init)
                        .in_set(ActionsSet::Build),
                    list_action_data_finalizer.after(ActionsSet::Approve),
                    init_action_request_building.in_set(ActionsSet::Init),
                    clear_action_building
                        .in_set(ActionsSet::Clear)
                        .before(ActionsSet::Init),
                )
                    .in_set(MainSet::Update),
            )
            .init_resource::<BuildingActions>()
            .init_resource::<ActionIncremented>()
            .init_resource::<ListActionDataRequests>()
            .init_resource::<ActionRequests>()
            .add_systems(
                FixedUpdate,
                incoming_messages
                    .before(ActionsSet::Init)
                    .in_set(MainSet::Update),
            )
            .add_event::<InputListActions>()
            .add_event::<InputAction>();
        }

        register_reliable_message::<ActionsClientMessage>(app, MessageSender::Client);
        register_reliable_message::<ActionsServerMessage>(app, MessageSender::Server);
    }
}
