use bevy::prelude::{Color, Component, Entity, Event, MouseButton, SystemSet};
use bevy::window::{PrimaryWindow, Window};
use bevy::{
    prelude::{Changed, Query},
    ui::Interaction,
};

pub const INPUT_TEXT_BG_PRESSED: Color = INPUT_TEXT_BG;
pub const INPUT_TEXT_BG: Color = Color::rgb(0.26, 0.3, 0.49);
pub const INPUT_TEXT_BG_HOVER: Color = Color::rgb(0.26, 0.3, 0.79);
pub const INPUT_TEXT_BG_FOCUSED: Color = Color::rgb(0.46, 0.5, 0.79);

/// The component for text input UI nodes.

#[derive(Component)]
pub struct TextInputNode {
    /// The text the input node currently holds.
    pub input: String,
    /// The placeholder text displayed.
    pub placeholder_text_option: Option<String>,
    /// Current text is placeholder, when focused the placeholder text clears.
    pub placeholder_active: bool,
    /// Apply a filter to allowed characters in the input field.
    pub character_filter_option: Option<CharacterFilter>,
    pub bg_color: Color,
    pub bg_color_hover: Color,
    pub bg_color_focused: Color,
}

impl Default for TextInputNode {
    fn default() -> Self {
        Self {
            input: String::default(),
            placeholder_text_option: Option::default(),
            placeholder_active: bool::default(),
            character_filter_option: Option::default(),
            bg_color: INPUT_TEXT_BG,
            bg_color_hover: INPUT_TEXT_BG_HOVER,
            bg_color_focused: INPUT_TEXT_BG_FOCUSED,
        }
    }
}

pub enum CharacterFilter {
    AccountName,
    ServerAddress,
    Chat,
    Integer,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum TextInputLabel {
    UiEvents,
    MousePressUnfocus,
}

use bevy::prelude::ResMut;
use bevy::prelude::With;
use bevy::ui::BackgroundColor;

/// UI event visuals.

pub fn ui_events(
    mut interaction_query: Query<
        (Entity, &Interaction, &TextInputNode, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    text_input: Res<TextInput>,
    mut focus: EventWriter<FocusTextInput>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary = primary_query.get_single().unwrap();

    for (entity, interaction, text_input_node, mut color) in interaction_query.iter_mut() {
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
                Interaction::Pressed => {
                    if !primary.cursor.visible {
                        continue;
                    }
                    *color = text_input_node.bg_color.into();
                    focus.send(FocusTextInput { entity });
                }
                Interaction::Hovered => {
                    if !primary.cursor.visible {
                        continue;
                    }
                    *color = text_input_node.bg_color_hover.into();
                }
                Interaction::None => {
                    *color = text_input_node.bg_color.into();
                }
            }
        } else {
            match *interaction {
                Interaction::Pressed => {
                    if !primary.cursor.visible {
                        continue;
                    }
                    *color = text_input_node.bg_color.into();
                    focus.send(FocusTextInput { entity });
                }
                _ => (),
            }
        }
    }
}
use bevy::prelude::warn;
use bevy::prelude::Children;
use bevy::prelude::Res;
use bevy::prelude::{Input, KeyCode};
use bevy::text::Text;
use bevy::{prelude::EventReader, window::ReceivedCharacter};

/// Event to unfocus the currently focused text input.
#[derive(Default, Event)]

pub struct UnfocusTextInput {
    pub entity_option: Option<Entity>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum FocusTextSet {
    Focus,
    Unfocus,
}

/// Event to focus a new text input.
#[derive(Event)]
pub struct FocusTextInput {
    pub entity: Entity,
}

/// Manages focus of text input.

pub(crate) fn focus_events(
    mut focus_events: EventReader<FocusTextInput>,
    mut unfocus_events: EventReader<UnfocusTextInput>,
    mut text_input: ResMut<TextInput>,
    mut input_query: Query<(&mut BackgroundColor, &TextInputNode, &Children)>,
    mut text_query: Query<&mut Text>,
) {
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
                            *old_color = input_node.bg_color.into();

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
        text_input.old_focus = text_input.focused_input.clone();
        text_input.focused_input = None;
    }
    for focus in focus_events.iter() {
        match text_input.focused_input {
            Some(entity) => {
                if entity != focus.entity {
                    match input_query.get_mut(entity) {
                        Ok((mut old_color, text_input_node, _)) => {
                            *old_color = text_input_node.bg_color.into();
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
            Ok((mut new_color, text_input_node, _)) => {
                *new_color = text_input_node.bg_color_focused.into();
            }
            Err(_) => {
                warn!("Couldnt find node of new text input focus. 0");
            }
        }
        text_input.old_focus = text_input.focused_input.clone();
        text_input.focused_input = Some(focus.entity);
    }
}

use bevy::prelude::EventWriter;

use bevy::time::Time;
use bevy::time::TimerMode;
use bevy::{prelude::Local, time::Timer};
use bevy_egui::EguiClipboard;
use resources::input::{KeyBind, KeyBinds, KeyCodeEnum};
use resources::ui::TextInput;
use std::time::Duration;

pub const COPY_PASTE_CTRL: &str = "CONTROL_COPY";
pub const COPY_PASTE_CTRL_RIGHT: &str = "CONTROL_COPY_RIGHT";
pub const COPY_PASTE_V: &str = "COPY_V";
pub const INPUT_BACK: &str = "INPUT_BACK";

pub(crate) fn register_input(mut map: ResMut<KeyBinds>) {
    map.list.insert(
        COPY_PASTE_CTRL.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::ControlLeft),
            description: "For copy pasting.".to_string(),
            name: "Control button.".to_string(),
            customizable: false,
        },
    );
    map.list.insert(
        COPY_PASTE_CTRL_RIGHT.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::ControlRight),
            description: "For copy pasting.".to_string(),
            name: "Right control button.".to_string(),
            customizable: false,
        },
    );
    map.list.insert(
        COPY_PASTE_V.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::V),
            description: "For copy pasting.".to_string(),
            name: "V button.".to_string(),
            customizable: false,
        },
    );
    map.list.insert(
        INPUT_BACK.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::Back),
            description: "For removing text from input.".to_string(),
            name: "Return key.".to_string(),
            customizable: false,
        },
    );
}

/// Register characters input and output as displayed text inside input node. Also manages ctrl+v paste.

pub(crate) fn input_characters(
    text_input: Res<TextInput>,
    mut backspace_timer: Local<Timer>,
    mut backspace_timer_not_first: Local<bool>,
    mut backspace_init_timer: Local<Timer>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut text_input_node_query: Query<(&mut TextInputNode, &Children)>,
    mut text_query: Query<&mut Text>,
    keys: Res<Input<KeyCode>>,
    keys2: Res<KeyBinds>,

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

                        if (keys.just_pressed(keys2.keyboard_bind(COPY_PASTE_CTRL)))
                            || keys.just_pressed(keys2.keyboard_bind(COPY_PASTE_CTRL_RIGHT))
                                && keys.just_pressed(keys2.keyboard_bind(COPY_PASTE_V))
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
                                                CharacterFilter::Chat => {
                                                    if (char.is_whitespace() && char != '\t')
                                                        || char.is_ascii_alphanumeric()
                                                        || char.is_ascii_graphic()
                                                    {
                                                        valid_char = true;
                                                    }
                                                }
                                                CharacterFilter::Integer => {
                                                    if char.is_numeric() {
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

                                    input_node.input = input_node.input.to_string();
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
                                        CharacterFilter::Chat => {
                                            if (ev.char.is_whitespace() && ev.char != '\t')
                                                || ev.char.is_ascii_alphanumeric()
                                                || ev.char.is_ascii_graphic()
                                            {
                                                valid_char = true;
                                            }
                                        }
                                        CharacterFilter::Integer => {
                                            if ev.char.is_numeric() {
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
                            input_node.input = input_node.input.to_string();
                        }

                        if keys.just_pressed(keys2.keyboard_bind(INPUT_BACK)) {
                            input_node.input.pop();

                            *backspace_init_timer =
                                Timer::new(Duration::from_millis(300), TimerMode::Once);
                        } else if keys.pressed(keys2.keyboard_bind(INPUT_BACK)) {
                            let delta_time = time.delta();
                            backspace_timer.tick(delta_time);
                            backspace_init_timer.tick(delta_time);

                            if backspace_init_timer.finished() {
                                if backspace_timer.paused() {
                                    *backspace_timer = Timer::new(
                                        Duration::from_millis(150),
                                        TimerMode::Repeating,
                                    );
                                }

                                if backspace_timer.just_finished() {
                                    input_node.input.pop();
                                }
                            }
                        } else if keys.just_released(keys2.keyboard_bind(INPUT_BACK)) {
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

/// Event that sets the content of the given TextInputNode
#[derive(Event)]
pub struct SetText {
    pub entity: Entity,
    pub text: String,
}
/// Processes [TextInputNode].

pub(crate) fn set_text_input_node_text(
    mut events: EventReader<SetText>,
    mut text_input_node_query: Query<(&mut TextInputNode, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for event in events.iter() {
        match text_input_node_query.get_mut(event.entity) {
            Ok((mut text_input_node, children)) => {
                text_input_node.input = event.text.clone();
                text_input_node.placeholder_active = false;
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

                match text_entity_option {
                    Some(e) => {
                        let mut text_component = text_query.get_mut(*e).unwrap();
                        match text_component.sections.get_mut(0) {
                            Some(section) => {
                                section.value = event.text.clone();
                            }
                            None => {
                                warn!("Couldnt find text section to set text of.");
                            }
                        }
                    }
                    None => {
                        warn!("Couldnt find text node to set text of.");
                    }
                }
            }
            Err(_) => {
                warn!("Couldnt find text input node to set text of.");
            }
        }
    }
}

use networking::server::IncomingReliableClientMessage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InterpolationSet {
    Main,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TextTree {
    Input,
}

use crate::net::UiClientMessage;
/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<UiClientMessage>>,
    mut text_tree_input_selection: EventWriter<TextTreeInputSelection>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            UiClientMessage::TextTreeInput(data) => {
                text_tree_input_selection.send(TextTreeInputSelection {
                    handle: message.handle,
                    id: data.id,
                    entry: data.entry,
                    entity: data.entity,
                });
            }
        }
    }
}
/// Client text tree input selection event.
#[derive(Event)]
pub struct TextTreeInputSelection {
    /// Handle of the submitter of the selection.
    pub handle: u64,
    /// Menu ID.
    pub id: String,
    /// The selection on the menu.
    pub entry: String,
    pub entity: Entity,
}

/// Manages focus of text input.

pub(crate) fn input_mouse_press_unfocus(
    buttons: Res<Input<MouseButton>>,
    text_input: Res<TextInput>,
    mut unfocus: EventWriter<UnfocusTextInput>,
    mut focus: EventReader<FocusTextInput>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let mut new_focus = None;
        for f in focus.iter() {
            new_focus = Some(f.entity);
        }
        let focus;

        match text_input.old_focus {
            Some(e) => {
                focus = Some(e);
            }
            None => {
                focus = text_input.focused_input;
            }
        }

        match focus {
            Some(e) => match new_focus {
                Some(x) => {
                    if e != x {
                        unfocus.send(UnfocusTextInput {
                            entity_option: Some(e),
                        });
                    }
                }
                None => {
                    unfocus.send(UnfocusTextInput {
                        entity_option: Some(e),
                    });
                }
            },
            None => {}
        }
    }
}

pub(crate) fn clear_old_focus(mut text_input: ResMut<TextInput>) {
    text_input.old_focus = None;
}
