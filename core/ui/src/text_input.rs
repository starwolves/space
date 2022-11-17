use bevy::prelude::{Color, Component, Entity, SystemLabel};
use bevy::{
    prelude::{Changed, Query},
    ui::{Interaction, UiColor},
};

pub const INPUT_TEXT_BG_PRESSED: Color = INPUT_TEXT_BG;
pub const INPUT_TEXT_BG: Color = Color::rgb(0.26, 0.3, 0.49);
pub const INPUT_TEXT_BG_HOVER: Color = Color::rgb(0.26, 0.3, 0.79);
pub const INPUT_TEXT_BG_FOCUSED: Color = Color::rgb(0.46, 0.5, 0.79);

/// The component for text input UI nodes.
#[cfg(feature = "client")]
#[derive(Component, Default)]
pub struct TextInputNode {
    pub input: String,
    pub placeholder_text_option: Option<String>,
    pub placeholder_active: bool,
    pub character_filter_option: Option<CharacterFilter>,
}
pub enum CharacterFilter {
    AccountName,
    ServerAddress,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "client")]
pub enum TextInputLabel {
    UiEvents,
    MousePressUnfocus,
}

#[cfg(feature = "client")]
#[derive(Default)]
pub struct TextInput {
    pub focused_input: Option<Entity>,
}
use bevy::prelude::ResMut;
use bevy::prelude::With;

/// UI event visuals.
#[cfg(feature = "client")]
pub(crate) fn ui_events(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<TextInputNode>),
    >,
    text_input: Res<TextInput>,
    mut focus: EventWriter<FocusTextInput>,
) {
    for (entity, interaction, mut color) in interaction_query.iter_mut() {
        let mut input_has_focus = false;
        match text_input.focused_input {
            Some(ent) => {
                if ent == entity {
                    input_has_focus = true;
                }
            }
            None => {}
        }

        if !input_has_focus {
            match *interaction {
                Interaction::Clicked => {
                    *color = INPUT_TEXT_BG_PRESSED.into();
                    focus.send(FocusTextInput { entity });
                }
                Interaction::Hovered => {
                    *color = INPUT_TEXT_BG_HOVER.into();
                }
                Interaction::None => {
                    *color = INPUT_TEXT_BG.into();
                }
            }
        }
    }
}
use bevy::prelude::warn;
use bevy::prelude::Children;
use bevy::prelude::MouseButton;
use bevy::prelude::Res;
use bevy::prelude::{Input, KeyCode};
use bevy::text::Text;
use bevy::{prelude::EventReader, window::ReceivedCharacter};

/// Event to unfocus the currently focused text input.
#[cfg(feature = "client")]
pub struct UnfocusTextInput {
    pub entity_option: Option<Entity>,
}

/// Event to focus a new text input.
#[cfg(feature = "client")]
pub struct FocusTextInput {
    pub entity: Entity,
}

/// Manages focus of text input.
#[cfg(feature = "client")]
pub(crate) fn focus_events(
    mut focus_events: EventReader<FocusTextInput>,
    mut unfocus_events: EventReader<UnfocusTextInput>,
    mut text_input: ResMut<TextInput>,
    mut input_query: Query<(&mut UiColor, &TextInputNode, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for focus in focus_events.iter() {
        match text_input.focused_input {
            Some(entity) => {
                if entity != focus.entity {
                    match input_query.get_mut(entity) {
                        Ok((mut old_color, _, _)) => {
                            *old_color = INPUT_TEXT_BG.into();
                        }
                        Err(_) => {
                            warn!("Couldnt find node of old text input focus. 1");
                        }
                    }
                }
            }
            None => {}
        }

        match input_query.get_mut(focus.entity) {
            Ok((mut new_color, _, _)) => {
                *new_color = INPUT_TEXT_BG_FOCUSED.into();
            }
            Err(_) => {
                warn!("Couldnt find node of new text input focus. 0");
            }
        }

        text_input.focused_input = Some(focus.entity);
    }

    for unfocus in unfocus_events.iter() {
        match text_input.focused_input {
            Some(entity) => {
                let mut should_unfocus = false;
                match unfocus.entity_option {
                    Some(e) => {
                        if entity == e {
                            should_unfocus = true;
                        }
                    }
                    None => {
                        should_unfocus = true;
                    }
                }
                if should_unfocus {
                    match input_query.get_mut(entity) {
                        Ok((mut old_color, input_node, children)) => {
                            *old_color = INPUT_TEXT_BG.into();

                            let mut text_entity_option = None;

                            for child in children.iter() {
                                match text_query.get(*child) {
                                    Ok(_) => {
                                        text_entity_option = Some(child);
                                        break;
                                    }
                                    Err(_) => {}
                                }
                            }

                            let mut text;

                            match text_entity_option {
                                Some(ent) => {
                                    text = text_query.get_mut(*ent).unwrap();
                                }
                                None => {
                                    warn!("Unfocus couldnt find text child node");
                                    continue;
                                }
                            }

                            match &input_node.placeholder_text_option {
                                Some(place_holder_text) => match text.sections.get_mut(0) {
                                    Some(t) => {
                                        if t.value.is_empty() {
                                            t.value = place_holder_text.to_string();
                                        }
                                    }
                                    None => {
                                        warn!("Unfocus couldnt find text section!");
                                    }
                                },
                                None => {}
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            None => {}
        }
        text_input.focused_input = None;
    }
}

use bevy::prelude::EventWriter;

/// Manages focus of text input.
#[cfg(feature = "client")]
pub(crate) fn input_mouse_press_unfocus(
    buttons: Res<Input<MouseButton>>,
    text_input: Res<TextInput>,
    mut unfocus: EventWriter<UnfocusTextInput>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        match text_input.focused_input {
            Some(e) => {
                unfocus.send(UnfocusTextInput {
                    entity_option: Some(e),
                });
            }
            None => {}
        }
    }
}
use bevy::time::Time;
use bevy::{prelude::Local, time::Timer};
use bevy_egui::EguiClipboard;
use std::time::Duration;

/// Register characters input and output as displayed text inside input node. Also manages ctrl+v paste.
#[cfg(feature = "client")]
pub(crate) fn input_characters(
    text_input: Res<TextInput>,
    mut backspace_timer: Local<Timer>,
    mut backspace_timer_not_first: Local<bool>,
    mut backspace_init_timer: Local<Timer>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut text_input_node_query: Query<(&mut TextInputNode, &Children)>,
    mut text_query: Query<&mut Text>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    clipboard: Res<EguiClipboard>,
    mut pasting: Local<bool>,
) {
    if !*backspace_timer_not_first {
        backspace_timer.pause();
        *backspace_timer_not_first = true;
    }

    let focused_input_node;

    match text_input.focused_input {
        Some(i) => {
            focused_input_node = i;
        }
        None => {
            return;
        }
    }

    match text_input_node_query.get_mut(focused_input_node) {
        Ok((mut input_node, children)) => {
            for child in children.iter() {
                match text_query.get_mut(*child) {
                    Ok(mut text_sections) => {
                        let text;

                        match text_sections.sections.get_mut(0) {
                            Some(t) => {
                                text = t;
                            }
                            None => {
                                warn!("Couldn't find the right text section!");
                                continue;
                            }
                        }

                        let mut is_pasting = false;

                        if (keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl))
                            && keys.pressed(KeyCode::V)
                        {
                            if !*pasting {
                                *pasting = true;
                                is_pasting = true;
                            }
                        } else {
                            *pasting = false;
                        }

                        if is_pasting {
                            match clipboard.get_contents() {
                                Some(clipboard_content) => {
                                    let mut validated_clipboard_text = "".to_string();

                                    for char in clipboard_content.chars() {
                                        let mut valid_char = false;

                                        match &input_node.character_filter_option {
                                            Some(char_filter) => match char_filter {
                                                CharacterFilter::AccountName => {
                                                    if char.is_ascii_alphanumeric() {
                                                        valid_char = true;
                                                    }
                                                }
                                                CharacterFilter::ServerAddress => {
                                                    if char.is_ascii_alphanumeric()
                                                        || char.is_ascii_graphic()
                                                    {
                                                        valid_char = true;
                                                    }
                                                }
                                            },
                                            None => {
                                                valid_char = true;
                                            }
                                        }

                                        if valid_char {
                                            validated_clipboard_text.push(char);
                                        }
                                    }

                                    input_node.input =
                                        input_node.input.clone() + &validated_clipboard_text;
                                }
                                None => {}
                            }
                        } else {
                            for ev in char_evr.iter() {
                                if input_node.placeholder_active {
                                    input_node.placeholder_active = false;
                                    text.value = "".to_string();
                                }

                                let mut valid_char = false;

                                match &input_node.character_filter_option {
                                    Some(char_filter) => match char_filter {
                                        CharacterFilter::AccountName => {
                                            if ev.char.is_ascii_alphanumeric() {
                                                valid_char = true;
                                            }
                                        }
                                        CharacterFilter::ServerAddress => {
                                            if ev.char.is_ascii_alphanumeric()
                                                || ev.char.is_ascii_graphic()
                                            {
                                                valid_char = true;
                                            }
                                        }
                                    },
                                    None => {
                                        valid_char = true;
                                    }
                                }
                                if valid_char {
                                    input_node.input.push(ev.char);
                                }
                            }
                        }

                        if keys.just_pressed(KeyCode::Back) {
                            input_node.input.pop();
                            *backspace_init_timer = Timer::new(Duration::from_millis(300), false);
                        } else if keys.pressed(KeyCode::Back) {
                            let delta_time = time.delta();
                            backspace_timer.tick(delta_time);
                            backspace_init_timer.tick(delta_time);

                            if backspace_init_timer.finished() {
                                if backspace_timer.paused() {
                                    *backspace_timer = Timer::new(Duration::from_millis(150), true);
                                }

                                if backspace_timer.just_finished() {
                                    input_node.input.pop();
                                }
                            }
                        } else if keys.just_released(KeyCode::Back) {
                            backspace_timer.pause();
                        }
                        text.value = input_node.input.clone();
                    }
                    Err(_) => {
                        warn!("Couldn't find Text child node.");
                    }
                }
            }
        }
        Err(_) => {
            warn!("Couldn't find focussed TextInputNode");
        }
    }
}
