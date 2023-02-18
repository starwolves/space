use bevy::{
    prelude::{
        AssetServer, BuildChildren, Color, Commands, Component, EventReader, NodeBundle, Res,
        ResMut, SystemLabel, TextBundle,
    },
    text::TextStyle,
    ui::{FlexDirection, Size, Style, Val},
};
use inventory::client::slots::AddedSlot;

use super::build::InventoryHudState;

#[derive(Component)]
pub struct SlotHud;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum InventoryHudLabels {
    UpdateSlot,
    QueueUpdate,
}

pub(crate) fn update_inventory_hud_slot(
    mut state: ResMut<InventoryHudState>,
    mut update_slot: EventReader<HudAddInventorySlot>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in update_slot.iter() {
        let width = (event.slot.slot.size.x as f32 / 16.) * 100.;
        let height = (event.slot.slot.size.y as f32 / 16.) * 100.;
        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");

        commands.entity(state.slots_node).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(width), Val::Percent((height * 0.5) + 10.)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
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
                    let slot_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(90.)),
                                ..Default::default()
                            },
                            background_color: Color::GRAY.into(),
                            ..Default::default()
                        })
                        .insert(SlotHud)
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
