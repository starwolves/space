use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use server::labels::ActionsLabels;

use crate::core::{
    clear_action_building, init_action_data_listing, init_action_request_building,
    list_action_data_finalizer, list_action_data_from_actions_component, ActionIncremented,
    ActionRequests, BuildingActions, ListActionDataRequests,
};

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
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
            .init_resource::<ActionRequests>();
    }
}
