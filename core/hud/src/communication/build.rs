use bevy::log::warn;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::{
        BuildChildren, ButtonBundle, Changed, Color, Commands, Component, Entity, EventWriter,
        NodeBundle, Query, Res, ResMut, Resource, TextBundle, With,
    },
    text::{TextSection, TextStyle},
    ui::{Display, FlexDirection, Interaction, Overflow, Style, Val},
};

use metadata::MetadataResource;
use resources::{hud::HudState, input::InputBuffer, ui::TextInput};
use ui::{
    button::SFButton,
    fonts::{Fonts, SOURCECODE_REGULAR_FONT},
    scrolling::ScrollingListInverted,
    text::COMMUNICATION_FONT_SIZE,
    text_input::{CharacterFilter, FocusTextInput, TextInputNode},
};

use crate::{input::binds::TOGGLE_CONSOLE_BIND, inventory::build::OpenHud};

use super::console::DisplayConsoleMessage;
#[derive(Component)]
pub struct ChatMessagesNode;
#[derive(Component)]
pub struct ChatMessagesBGNode;
#[derive(Component)]
pub struct ConsoleMessagesNode;
#[derive(Component)]
pub struct ConsoleMessagesBGNode;
#[derive(Resource)]
pub struct HudCommunicationState {
    pub chat_messages_node: Entity,
    pub console_messages_node: Entity,
    pub console_messages_bg_node: Entity,
    pub communication_input_node: Entity,
    pub communication_input_focused: bool,
    pub is_displaying_console: bool,
    pub chat_messages_bg_node: Entity,
}
#[derive(Component)]
pub struct CommunicationInputNode;
#[derive(Component)]
pub struct ToggleConsoleButton;

pub(crate) fn toggle_console_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ToggleConsoleButton>)>,
    mut state: ResMut<HudCommunicationState>,
    mut style_query: Query<&mut Style>,
    keys: Res<InputBuffer>,
    mut focus_event: EventWriter<FocusTextInput>,
    mut open_hud: EventWriter<OpenHud>,
    text_input: Res<TextInput>,
) {
    if keys.just_pressed(TOGGLE_CONSOLE_BIND) && text_input.focused_input.is_none() {
        state.is_displaying_console = true;
        match style_query.get_mut(state.chat_messages_bg_node) {
            Ok(mut style) => {
                style.display = Display::None;
            }
            Err(_) => {
                warn!("Couldnt find visibility component of chat messages node.");
            }
        }
        match style_query.get_mut(state.console_messages_bg_node) {
            Ok(mut style) => {
                style.display = Display::Flex;
            }
            Err(_) => {
                warn!("Couldnt find visibility component of console messages node.");
            }
        }
        focus_event.send(FocusTextInput {
            entity: state.communication_input_node,
        });
        open_hud.send(OpenHud { open: true });
    }

    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                state.is_displaying_console = !state.is_displaying_console;

                match style_query.get_mut(state.chat_messages_bg_node) {
                    Ok(mut style) => {
                        if state.is_displaying_console {
                            style.display = Display::None;
                        } else {
                            style.display = Display::Flex;
                        }
                    }
                    Err(_) => {
                        warn!("Couldnt find visibility component of chat messages node.");
                    }
                }
                match style_query.get_mut(state.console_messages_bg_node) {
                    Ok(mut style) => {
                        if state.is_displaying_console {
                            style.display = Display::Flex;
                        } else {
                            style.display = Display::None;
                        }
                    }
                    Err(_) => {
                        warn!("Couldnt find visibility component of console messages node.");
                    }
                }
            }
            _ => (),
        }
    }
}

pub(crate) fn build_communication_ui(
    hud_state: Res<HudState>,
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
    let sourcecode_font = fonts.handles.get(SOURCECODE_REGULAR_FONT).unwrap();

    let mut chat_messages_node = None;
    let mut chat_messages_bg_node = None;

    let mut console_messages_node = None;
    let mut communication_input_node = None;
    let mut console_messages_bg_node = None;
    commands
        .entity(hud_state.left_edge_node)
        .with_children(|parent| {
            chat_messages_bg_node = Some(
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(35.),
                            flex_direction: FlexDirection::ColumnReverse,
                            overflow: Overflow::clip(),

                            ..Default::default()
                        },
                        background_color: Color::rgba(0.0, 0.0, 1.0, 0.05).into(),
                        ..Default::default()
                    })
                    .insert(ChatMessagesBGNode)
                    .with_children(|parent| {
                        chat_messages_node = Some(
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::ColumnReverse,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    ScrollingListInverted::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .insert(ChatMessagesNode)
                                .id(),
                        );
                    })
                    .id(),
            );

            console_messages_bg_node = Some(
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(35.),
                            flex_direction: FlexDirection::ColumnReverse,
                            display: Display::None,
                            overflow: Overflow::clip(),
                            ..Default::default()
                        },
                        background_color: Color::rgba(0.25, 0.25, 0.25, 0.4).into(),

                        ..Default::default()
                    })
                    .insert(ConsoleMessagesBGNode)
                    .with_children(|parent| {
                        console_messages_node = Some(
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::ColumnReverse,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    ScrollingListInverted::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .insert(ConsoleMessagesNode)
                                .id(),
                        );
                    })
                    .id(),
            );
        });

    commands
        .entity(hud_state.bottom_edge_node)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(27.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let text = "...".to_string();
                    let mut builder = parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(50.),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    communication_input_node = Some(builder.id());
                    builder.insert((
                        TextInputNode {
                            placeholder_active: true,
                            character_filter_option: Some(CharacterFilter::Chat),
                            placeholder_text_option: Some(text.to_owned()),
                            bg_color: Color::rgba(0.26, 0.3, 0.49, 0.5),
                            bg_color_focused: Color::rgba(0.46, 0.5, 0.79, 0.5),
                            bg_color_hover: Color::rgba(0.26, 0.3, 0.79, 0.5),
                            ..Default::default()
                        },
                        Interaction::default(),
                        CommunicationInputNode,
                    ));

                    builder.with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                font: sourcecode_font.clone(),
                                font_size: COMMUNICATION_FONT_SIZE,
                                color: Color::WHITE.into(),
                            },
                        ));
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(3.3),
                                height: Val::Percent(25.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: Color::DARK_GRAY.into(),
                                        style: Style {
                                            width: Val::Percent(100.),
                                            height: Val::Percent(100.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    ToggleConsoleButton,
                                    SFButton::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "~",
                                        TextStyle {
                                            font: sourcecode_font.clone(),
                                            font_size: 16.0,
                                            color: Color::WHITE.into(),
                                        },
                                    ));
                                });
                        });
                });
        });
    commands.insert_resource(HudCommunicationState {
        chat_messages_node: chat_messages_node.unwrap(),
        communication_input_node: communication_input_node.unwrap(),
        communication_input_focused: false,
        console_messages_node: console_messages_node.unwrap(),
        is_displaying_console: false,
        console_messages_bg_node: console_messages_bg_node.unwrap(),
        chat_messages_bg_node: chat_messages_bg_node.unwrap(),
    });
}

pub const CONSOLE_FONT_COLOR: Color = Color::WHITE;

pub(crate) fn console_welcome_message(
    mut events: EventWriter<DisplayConsoleMessage>,
    fonts: Res<Fonts>,
    meta: Res<MetadataResource>,
) {
    let mut welcome_message = "".to_string();

    match &meta.data {
        Some(data) => {
            let mut sf_version_option = None;
            let mut bevy_version_option = None;

            for package in data.packages.iter() {
                if package.name == "bevy" {
                    bevy_version_option = Some(package.version.clone());
                } else if package.name == "app" {
                    sf_version_option = Some(package.version.clone());
                }
            }

            if sf_version_option.is_none() || bevy_version_option.is_none() {
                warn!("Couldnt find bevy or app packages");
                return;
            }

            let sf_version = sf_version_option.unwrap();
            let bevy_version = bevy_version_option.unwrap();
            welcome_message = welcome_message
                + &format!(
                    "Space Frontiers v{}.{}.{}\n",
                    sf_version.major, sf_version.minor, sf_version.patch
                )
                + &format!(
                    "Bevy v{}.{}.{}\n",
                    bevy_version.major, bevy_version.minor, bevy_version.patch
                );
        }
        None => {}
    }

    match &meta.commit {
        Some(c) => {
            welcome_message = welcome_message + &format!("Commit: {}\n", c);
        }
        None => {}
    }

    welcome_message = welcome_message + "Write \"help\" for a list of available commands.";

    events.send(DisplayConsoleMessage {
        sections: vec![TextSection::new(
            welcome_message,
            TextStyle {
                font: fonts.handles.get(SOURCECODE_REGULAR_FONT).unwrap().clone(),
                font_size: COMMUNICATION_FONT_SIZE,
                color: CONSOLE_FONT_COLOR.into(),
            },
        )],
    });
}

pub const MESSAGES_DEFAULT_MAX_WIDTH: f32 = 380.;
pub const MESSAGES_DEFAULT_MIN_WIDTH: f32 = 100.;
