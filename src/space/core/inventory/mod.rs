use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};

use self::events::{inventory_actions, net_system};
use self::{
    entity_update::inventory_update,
    events::{
        InputDropCurrentItem, InputSwitchHands, InputTakeOffItem, InputThrowItem,
        InputUseWorldItem, InputWearItem, NetDropCurrentItem, NetPickupWorldItem, NetSwitchHands,
        NetTakeOffItem, NetThrowItem, NetWearItem,
    },
    systems::{
        drop_current_item::drop_current_item, inventory_tab_data::inventory_tab_data,
        pickup_world_item::pickup_world_item, switch_hands::switch_hands,
        take_off_item::take_off_item, throw_item::throw_item, wear_item::wear_item,
    },
};
use crate::space::{PostUpdateLabels, UpdateLabels};

use super::tab_actions::TabActionsQueueLabels;

pub mod components;
pub mod entity_update;
pub mod events;
pub mod systems;

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
            .add_system(inventory_actions.after(TabActionsQueueLabels::TabAction))
            .add_system_to_stage(
                PostUpdate,
                net_system.after(PostUpdateLabels::VisibleChecker),
            );
    }
}
