use bevy::{
    prelude::{
        warn, AssetServer, BuildChildren, Color, Commands, Component, EventReader, NodeBundle,
        Query, Res, ResMut, SystemLabel, TextBundle, With,
    },
    text::TextStyle,
    ui::{FlexDirection, Node, Size, Style, Val},
};
use inventory::client::slots::AddedSlot;
use math::grid::Vec2Int;

use super::build::{InventoryHudRootNode, InventoryHudState};

#[derive(Component)]
pub struct SlotHud {
    pub size: Vec2Int,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum InventoryHudLabels {
    UpdateSlot,
    QueueUpdate,
}
#[derive(Component)]
pub struct SlotScaler {
    pub size: Vec2Int,
}

pub(crate) fn scale_slots(
    mut slot_query: Query<(&SlotScaler, &mut Style)>,
    root_node_query: Query<&Node, With<InventoryHudRootNode>>,
    state: Res<InventoryHudState>,
) {
    for (slot_scaler, mut style) in slot_query.iter_mut() {
        let width = (slot_scaler.size.x as f32 / 16.) * 100.;

        let root_size;

        match root_node_query.get(state.root_node) {
            Ok(c) => {
                root_size = c.size();
            }
            Err(_) => {
                warn!("Couldnt find root node.");
                continue;
            }
        }

        let scaler = slot_scaler.size.y as f32 / slot_scaler.size.x as f32;

        let x_multiplier = 1. + (root_size.x / (512. * 0.25));
        let new_width = width * x_multiplier;
        let new_height = new_width * scaler;

        style.size = Size::new(Val::Px(new_width), Val::Px(new_height));
    }
}

pub(crate) fn update_inventory_hud_slot(
    mut state: ResMut<InventoryHudState>,
    mut update_slot: EventReader<HudAddInventorySlot>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in update_slot.iter() {
        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");

        commands.entity(state.slots_node).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(SlotScaler {
                    size: event.slot.slot.size,
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Px(16.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                event.slot.slot.name.clone() + ":",
                                TextStyle {
                                    font: arizone_font.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // The inventory grid space.
                    let mut gray = Color::GRAY;
                    gray.set_a(0.7);
                    let slot_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),

                                ..Default::default()
                            },
                            background_color: gray.into(),
                            ..Default::default()
                        })
                        .insert(SlotHud {
                            size: event.slot.slot.size,
                        })
                        .id();
                    state.slots.insert(event.slot.id, slot_entity);
                });
        });
    }
}

pub struct HudAddInventorySlot {
    pub slot: AddedSlot,
}

#[derive(Component)]
pub struct InventorySlotsNode;
