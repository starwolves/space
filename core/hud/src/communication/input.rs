use bevy::{
    prelude::{
        warn, Children, Event, EventReader, EventWriter, Input, KeyCode, Query, Res, ResMut,
        SystemSet,
    },
    text::Text,
    ui::{Display, Style},
};
use chat::net::ChatClientMessage;
use console_commands::net::ClientSideConsoleInput;
use networking::client::OutgoingReliableClientMessage;
use resources::{
    binds::KeyBinds,
    hud::{EscapeMenuState, HudState},
    input::InputBuffer,
    ui::{MainMenuState, TextInput},
};
use ui::text_input::{FocusTextInput, TextInputNode, UnfocusTextInput};

use crate::{
    input::binds::{SUBMIT_CONSOLE_BIND, TOGGLE_CHAT},
    inventory::build::{InventoryHudState, OpenHud},
};

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ConsoleCommandsClientSet {
    Submit,
    Display,
}

use super::build::HudCommunicationState;

pub(crate) fn text_input(
    keyboard: Res<InputBuffer>,
    text_input_state: Res<TextInput>,
    mut text_node_query: Query<(&mut TextInputNode, &Children)>,
    mut text_node_input_query: Query<&mut Text>,
    mut net: EventWriter<OutgoingReliableClientMessage<ChatClientMessage>>,
    state: Res<HudCommunicationState>,
    mut console: EventWriter<ClientSideConsoleInput>,
) {
    match text_input_state.focused_input {
        Some(focused_input_entity) => {
            if keyboard.just_pressed(SUBMIT_CONSOLE_BIND) {
                match text_node_query.get_mut(focused_input_entity) {
                    Ok((mut text_input_component, children)) => {
                        for child in children {
                            match text_node_input_query.get_mut(*child) {
                                Ok(mut text) => {
                                    let input_text = text_input_component.input.trim().to_string();
                                    if input_text.is_empty() {
                                        continue;
                                    }
                                    for section in text.sections.iter_mut() {
                                        section.value = "".to_string();
                                        text_input_component.input = "".to_string();
                                    }
                                    if state.is_displaying_console {
                                        console
                                            .send(ClientSideConsoleInput::from_string(input_text));
                                    } else {
                                        net.send(OutgoingReliableClientMessage {
                                            message: ChatClientMessage::InputChatMessage(
                                                input_text,
                                            ),
                                        });
                                    }
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
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum CommunicationToggleSet {
    Toggle,
}
#[derive(Event)]
pub(crate) struct ToggleCommunication;

pub(crate) fn tab_communication_input_toggle(
    keys: Res<Input<KeyCode>>,
    binds: Res<KeyBinds>,
    mut event: EventWriter<ToggleCommunication>,
) {
    if keys.just_pressed(binds.bind(TOGGLE_CHAT)) {
        event.send(ToggleCommunication);
    }
}

pub(crate) fn communication_input_toggle(
    mut events: EventReader<ToggleCommunication>,
    mut state: ResMut<HudCommunicationState>,
    mut open_hud: EventWriter<OpenHud>,
    mut style_query: Query<&mut Style>,

    mut focus_event: EventWriter<FocusTextInput>,
    mut unfocus_event: EventWriter<UnfocusTextInput>,
    hud_state: Res<HudState>,
    text_input: Res<TextInput>,
    esc_state: Res<EscapeMenuState>,
    inv_state: Res<InventoryHudState>,
    main_mnu_state: Res<MainMenuState>,
) {
    if esc_state.visible || inv_state.open || main_mnu_state.enabled {
        return;
    }
    for _ in events.iter() {
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

        if state.is_displaying_console && text_input.focused_input.is_none() {
            state.is_displaying_console = false;
            match style_query.get_mut(state.chat_messages_bg_node) {
                Ok(mut style) => {
                    style.display = Display::Flex;
                }
                Err(_) => {
                    warn!("Couldnt find visibility component of chat messages node.");
                }
            }
            match style_query.get_mut(state.console_messages_bg_node) {
                Ok(mut style) => {
                    style.display = Display::None;
                }
                Err(_) => {
                    warn!("Couldnt find visibility component of console messages node.");
                }
            }
        }
    }
}
