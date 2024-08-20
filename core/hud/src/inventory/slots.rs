use bevy::color::Srgba;
use bevy::log::warn;
use bevy::{
    prelude::{
        BuildChildren, Color, Commands, Component, Event, EventReader, NodeBundle, Query, Res,
        ResMut, SystemSet, TextBundle, With,
    },
    text::TextStyle,
    ui::{FlexDirection, Node, Style, Val},
};
use inventory::client::slots::AddedSlot;
use resources::math::Vec2Int;
use ui::fonts::{Fonts, EMPIRE_FONT};

use super::build::{InventoryHudRootNode, InventoryHudState};

#[derive(Component)]
pub struct SlotHud {
    pub size: Vec2Int,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum InventoryHudSet {
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

        style.width = Val::Px(new_width);
        style.height = Val::Px(new_height);
    }
}

pub(crate) fn update_inventory_hud_slot(
    mut state: ResMut<InventoryHudState>,
    mut update_slot: EventReader<HudAddInventorySlot>,
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
    for event in update_slot.read() {
        let empire_font = fonts.handles.get(EMPIRE_FONT).unwrap();

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
                                width: Val::Percent(100.),
                                height: Val::Px(16.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                event.slot.slot.name.clone() + ":",
                                TextStyle {
                                    font: empire_font.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // The inventory grid space.
                    let mut gray = bevy::color::palettes::css::GRAY;
                    gray = bevy::prelude::Color::Srgba(Srgba {
                        alpha: 0.7,
                        ..Srgba::from(gray)
                    })
                    .into();
                    let slot_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Px(200.),
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
#[derive(Event)]
pub struct HudAddInventorySlot {
    pub slot: AddedSlot,
}

#[derive(Component)]
pub struct InventorySlotsNode;
