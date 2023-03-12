use bevy::{
    prelude::{
        warn, AssetServer, BuildChildren, Children, Commands, EventReader, EventWriter, Input,
        KeyCode, NodeBundle, Query, Res, TextBundle,
    },
    text::{Text, TextSection, TextStyle},
    ui::{Size, Style, Val},
};
use chat::net::{ChatClientMessage, ChatServerMessage};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use resources::{hud::HudState, ui::TextInput};
use ui::{
    fonts::Fonts,
    text_input::{FocusTextInput, TextInputNode, UnfocusTextInput},
};

use crate::inventory::build::OpenHud;

use super::build::HudCommunicationState;

pub(crate) fn text_input(
    keyboard: Res<Input<KeyCode>>,
    text_input_state: Res<TextInput>,
    mut text_node_query: Query<(&mut TextInputNode, &Children)>,
    mut text_node_input_query: Query<&mut Text>,
    mut net: EventWriter<OutgoingReliableClientMessage<ChatClientMessage>>,
) {
    match text_input_state.focused_input {
        Some(focused_input_entity) => {
            if keyboard.just_pressed(KeyCode::Return) {
                match text_node_query.get_mut(focused_input_entity) {
                    Ok((mut text_input_component, children)) => {
                        for child in children {
                            match text_node_input_query.get_mut(*child) {
                                Ok(mut text) => {
                                    let input_text = text_input_component.input.clone();

                                    for section in text.sections.iter_mut() {
                                        section.value = "".to_string();
                                        text_input_component.input = "".to_string();
                                    }

                                    net.send(OutgoingReliableClientMessage {
                                        message: ChatClientMessage::InputChatMessage(input_text),
                                    });
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {
                        warn!("Could not get text input entity");
                    }
                }
            }
        }
        None => {}
    }
}

pub(crate) fn display_global_chat_message(
    mut net: EventReader<IncomingReliableServerMessage<ChatServerMessage>>,
    chat_state: Res<HudCommunicationState>,
    mut commands: Commands,
    fonts: Res<Fonts>,
    asset_server: Res<AssetServer>,
) {
    for message in net.iter() {
        match &message.message {
            ChatServerMessage::ChatMessage(message) => {
                let mut sections = vec![];

                for net_section in message.sections.iter() {
                    sections.push(TextSection::new(
                        net_section.text.clone(),
                        TextStyle {
                            font: asset_server
                                .load(fonts.map.get(&net_section.font).expect("Font not loaded")),
                            font_size: net_section.font_size,
                            color: net_section.color,
                        },
                    ));
                }

                let text_section = commands
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_sections(sections));
                    })
                    .id();

                commands
                    .entity(chat_state.chat_messages_node)
                    .insert_children(0, &[text_section]);
            }
        }
    }
}

pub(crate) fn tab_communication_input_toggle(
    keys: Res<Input<KeyCode>>,
    state: Res<HudCommunicationState>,
    mut open_hud: EventWriter<OpenHud>,

    mut focus_event: EventWriter<FocusTextInput>,
    mut unfocus_event: EventWriter<UnfocusTextInput>,
    hud_state: Res<HudState>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        let is_focused = hud_state.expanded;

        if is_focused {
            unfocus_event.send(UnfocusTextInput {
                entity_option: Some(state.communication_input_node),
            });
        } else {
            focus_event.send(FocusTextInput {
                entity: state.communication_input_node,
            });
        }

        open_hud.send(OpenHud { open: !is_focused });
    }
}
