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
    mut input_query: Query<&mut UiColor, With<TextInputNode>>,
) {
    for focus in focus_events.iter() {
        match text_input.focused_input {
            Some(entity) => {
                if entity != focus.entity {
                    match input_query.get_mut(entity) {
                        Ok(mut old_color) => {
                            *old_color = INPUT_TEXT_BG.into();
                        }
                        Err(_) => {
                            warn!("Couldnt find node of old text input focus.");
                        }
                    }
                }
            }
            None => {}
        }

        match input_query.get_mut(focus.entity) {
            Ok(mut new_color) => {
                *new_color = INPUT_TEXT_BG_FOCUSED.into();
            }
            Err(_) => {
                warn!("Couldnt find node of new text input focus.");
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
                        Ok(mut old_color) => {
                            *old_color = INPUT_TEXT_BG.into();
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

/// Register characters input and output as displayed text inside input node.
#[cfg(feature = "client")]
pub(crate) fn input_characters(
    text_input: Res<TextInput>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut text_input_node_query: Query<(&mut TextInputNode, &Children)>,
    mut text_query: Query<&mut Text>,
    keys: Res<Input<KeyCode>>,
) {
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

                        if keys.just_pressed(KeyCode::Back) {
                            input_node.input.pop();
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
