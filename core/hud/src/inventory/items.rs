use bevy::{
    prelude::{
        info, warn, BuildChildren, Color, Commands, EventReader, EventWriter, NodeBundle, Res,
    },
    ui::{Size, Style, Val},
};
use entity::{
    entity_types::{EntityType, EntityTypes},
    spawn::EntityBuildData,
};
use inventory::{
    server::inventory::{Inventory, ItemAddedToSlot},
    spawn_item::InventoryItemBuilder,
};

use crate::inventory::queue::RequeueHudAddItemToSlot;

use super::build::InventoryHudState;

pub(crate) fn requeue_hud_add_item_to_slot(
    mut i_events: EventReader<RequeueHudAddItemToSlot>,
    mut o_events: EventWriter<HudAddItemToSlot>,
) {
    for event in i_events.iter() {
        o_events.send(event.queued.clone());
    }
}

pub fn update_inventory_hud_add_item_to_slot<
    T: InventoryItemBuilder + EntityType + Default + Send + Sync + 'static,
>(
    mut update_item: EventReader<HudAddItemToSlot>,
    mut queue: EventWriter<RequeueHudAddItemToSlot>,
    mut commands: Commands,
    state: Res<InventoryHudState>,
    types: Res<EntityTypes>,
    inventory: Res<Inventory>,
) {
    for event in update_item.iter() {
        let slot_entity;
        match state.slots.get(&event.item.slot_id) {
            Some(ent) => {
                slot_entity = *ent;
            }
            None => {
                info!("Queueing HudAddItemToSlot..");
                queue.send(RequeueHudAddItemToSlot {
                    queued: event.clone(),
                });
                continue;
            }
        }
        let mut type_id_option = None;
        for (id, n) in types.netcode_types.iter() {
            if &event.item.item_type_id == n {
                type_id_option = Some(id.clone());
                break;
            }
        }
        let item_type_identity;
        match type_id_option {
            Some(i) => {
                item_type_identity = i;
            }
            None => {
                warn!("Couldnt find entity id from netcode type.");
                continue;
            }
        }

        let identity = T::default();

        if item_type_identity != identity.to_string() {
            continue;
        }

        let slot;
        match inventory.slots.get(&event.item.slot_id) {
            Some(s) => {
                slot = s;
            }
            None => {
                warn!("Couldnt fgind slot!");
                continue;
            }
        }

        let inventory_item_bundle = T::default().get_bundle(&EntityBuildData::default());

        let width = (inventory_item_bundle.inventory_item.slot_size.x as f32 / 16.) * 100.;
        let height = ((inventory_item_bundle.inventory_item.slot_size.y as f32 / 16.) * 100.)
            * (slot.size.x as f32 / slot.size.y as f32);

        commands.entity(slot_entity).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(width), Val::Percent(height)),
                    ..Default::default()
                },
                background_color: Color::BEIGE.into(),
                ..Default::default()
            });
        });
    }
}
#[derive(Clone)]
pub struct HudAddItemToSlot {
    pub item: ItemAddedToSlot,
}
