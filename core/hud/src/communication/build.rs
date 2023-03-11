use bevy::{
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Color, Commands, Component, Entity, NodeBundle,
        Res, Resource, TextBundle,
    },
    text::TextStyle,
    ui::{FlexDirection, Interaction, Size, Style, Val},
};
use resources::hud::HudState;
use ui::{
    button::ButtonVisuals,
    fonts::EMPIRE_FONT,
    text_input::{CharacterFilter, TextInputNode},
};
#[derive(Component)]
pub struct ChatMessagesNode;

#[derive(Resource)]
pub struct HudCommunicationState {
    pub chat_messages_node: Entity,
    pub communication_input_node: Entity,
    pub communication_input_focused: bool,
}
#[derive(Component)]
pub struct CommunicationInputNode;

pub(crate) fn build_communication_ui(
    hud_state: Res<HudState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let empire_font = asset_server.load(EMPIRE_FONT);

    let mut chat_messages_node = Entity::from_bits(0);
    let mut communication_input_node = Entity::from_bits(0);
    commands
        .entity(hud_state.left_edge_node)
        .with_children(|parent| {
            chat_messages_node = parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(35.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ChatMessagesNode)
                .id();
        });

    commands
        .entity(hud_state.bottom_edge_node)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(29.18), Val::Percent(100.)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let text = "...".to_string();
                    let mut builder = parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(50.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    communication_input_node = builder.id();
                    builder.insert((
                        TextInputNode {
                            placeholder_active: true,
                            character_filter_option: Some(CharacterFilter::Chat),
                            placeholder_text_option: Some(text.to_owned()),
                            ..Default::default()
                        },
                        Interaction::default(),
                        CommunicationInputNode,
                    ));

                    builder.with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                font: empire_font.clone(),
                                font_size: 8.,
                                color: Color::WHITE.into(),
                            },
                        ));
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(3.3), Val::Percent(25.)),

                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: Color::DARK_GRAY.into(),
                                        ..Default::default()
                                    },
                                    ButtonVisuals::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "~",
                                        TextStyle {
                                            font: empire_font.clone(),
                                            font_size: 16.0,
                                            color: Color::WHITE.into(),
                                        },
                                    ));
                                });
                        });
                });
        });
    commands.insert_resource(HudCommunicationState {
        chat_messages_node,
        communication_input_node,
        communication_input_focused: false,
    });
}
