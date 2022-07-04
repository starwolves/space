use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};

use crate::core::{
    space_plugin::plugin::{PostUpdateLabels, UpdateLabels},
    tab_actions::plugin::TabActionsQueueLabels,
};

use super::{
    actions::actions,
    entity_update::inventory_update,
    inventory::{
        switch_hands, InputDropCurrentItem, InputSwitchHands, InputTakeOffItem, InputThrowItem,
        InputUseWorldItem, InputWearItem,
    },
    inventory_tab_data::inventory_tab_data,
    item_events::{drop_current_item, pickup_world_item, take_off_item, throw_item, wear_item},
    net::{
        net_system, NetDropCurrentItem, NetPickupWorldItem, NetSwitchHands, NetTakeOffItem,
        NetThrowItem, NetWearItem,
    },
};

use bevy::app::CoreStage::PostUpdate;
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputUseWorldItem>()
            .add_event::<NetPickupWorldItem>()
            .add_event::<InputDropCurrentItem>()
            .add_event::<NetDropCurrentItem>()
            .add_event::<InputSwitchHands>()
            .add_event::<NetSwitchHands>()
            .add_event::<InputWearItem>()
            .add_event::<NetWearItem>()
            .add_event::<InputTakeOffItem>()
            .add_event::<NetTakeOffItem>()
            .add_event::<InputThrowItem>()
            .add_event::<NetThrowItem>()
            .add_system(pickup_world_item)
            .add_system(switch_hands)
            .add_system(wear_item)
            .add_system(take_off_item)
            .add_system(inventory_tab_data)
            .add_system(throw_item)
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(inventory_update),
            )
            .add_system(drop_current_item.label(UpdateLabels::DropCurrentItem))
            .add_system(actions.after(TabActionsQueueLabels::TabAction))
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            );
    }
}
