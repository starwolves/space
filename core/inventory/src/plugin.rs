use api::data::{ActionsLabels, PostUpdateLabels, UpdateLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;

use crate::{actions::pickup_prerequisite_check, item_events::pickup_world_item_action};

use super::{
    entity_update::inventory_update,
    item_events::{drop_current_item, pickup_world_item, take_off_item, throw_item, wear_item},
    net::{
        NetDropCurrentItem, NetPickupWorldItem, NetSwitchHands, NetTakeOffItem, NetThrowItem,
        NetWearItem,
    },
    switch_hands::switch_hands,
};

use bevy::app::CoreStage::PostUpdate;
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetPickupWorldItem>()
            .add_event::<NetDropCurrentItem>()
            .add_event::<NetSwitchHands>()
            .add_event::<NetWearItem>()
            .add_event::<NetTakeOffItem>()
            .add_event::<NetThrowItem>()
            .add_system(pickup_world_item)
            .add_system(switch_hands)
            .add_system(wear_item)
            .add_system(take_off_item)
            .add_system(throw_item)
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(inventory_update),
            )
            .add_system(drop_current_item.label(UpdateLabels::DropCurrentItem))
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetPickupWorldItem>)
                    .with_system(net_system::<NetDropCurrentItem>)
                    .with_system(net_system::<NetSwitchHands>)
                    .with_system(net_system::<NetWearItem>)
                    .with_system(net_system::<NetTakeOffItem>)
                    .with_system(net_system::<NetThrowItem>),
            )
            .add_system(
                pickup_prerequisite_check
                    .label(ActionsLabels::Approve)
                    .after(ActionsLabels::Init),
            )
            .add_system(
                pickup_world_item_action
                    .label(ActionsLabels::Action)
                    .after(ActionsLabels::Approve),
            );
    }
}
