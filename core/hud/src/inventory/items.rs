use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Color, Commands, Component, EventReader,
        EventWriter, ImageBundle, NodeBundle, Res,
    },
    ui::{PositionType, Size, Style, UiImage, UiRect, Val},
};
use entity::{
    entity_types::{EntityType, EntityTypes},
    spawn::EntityBuildData,
};
use inventory::{
    server::inventory::{Inventory, ItemAddedToSlot},
    spawn_item::InventoryItemBuilder,
};
use math::grid::Vec2Int;

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

#[derive(Component)]
pub struct SlotItemHudPositioner {
    pub position: Vec2Int,
    pub slot_id: u8,
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
    asset_server: Res<AssetServer>,
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

        if item_type_identity != identity.get_identity() {
            continue;
        }

        let slot;
        match inventory.slots.get(&event.item.slot_id) {
            Some(s) => {
                slot = s;
            }
            None => {
                warn!("Couldnt find slot!");
                continue;
            }
        }

        let entity_type = T::default();

        let inventory_item_bundle = entity_type.get_bundle(&EntityBuildData::default());

        let width = (inventory_item_bundle.inventory_item.slot_size.x as f32 / 16.) * 100.;
        let height = ((inventory_item_bundle.inventory_item.slot_size.y as f32 / 16.) * 100.)
            * (slot.size.x as f32 / slot.size.y as f32);

        let mut corrected_item_position = event.item.position.clone();
        corrected_item_position.x += slot.size.x / 2;
        corrected_item_position.y += slot.size.y / 2;
        corrected_item_position.y -= inventory_item_bundle.inventory_item.slot_size.y;

        let x = corrected_item_position.x as f32 * 12.5;
        let y = corrected_item_position.y as f32 * 12.5;

        let item_image = asset_server
            .load("entities/".to_string() + &entity_type.get_clean_identity() + "/item.png");

        let item_space_color = Color::rgba(60. / 255., 0., 226. / 255., 0.7);

        commands.entity(slot_entity).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(width), Val::Percent(height)),
                        position_type: PositionType::Absolute,
                        position: UiRect::new(
                            Val::Percent(x),
                            Val::Undefined,
                            Val::Undefined,
                            Val::Percent(y),
                        ),
                        ..Default::default()
                    },
                    background_color: item_space_color.into(),
                    ..Default::default()
                })
                .insert(SlotItemHudPositioner {
                    position: event.item.position.clone(),
                    slot_id: event.item.slot_id,
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            ..Default::default()
                        },
                        image: UiImage::from(item_image),
                        ..Default::default()
                    });
                });
        });
    }
}
#[derive(Clone)]
pub struct HudAddItemToSlot {
    pub item: ItemAddedToSlot,
}
