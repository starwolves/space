use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::{
        BuildChildren, Commands, Component, Event, EventReader, EventWriter, Label, NodeBundle,
        Res, TextBundle,
    },
    text::{TextSection, TextStyle},
    ui::{FlexDirection, Style, Val},
};
use console_commands::net::{
    ClientSideConsoleInput, ConsoleCommandsClientMessage, ConsoleCommandsServerMessage,
};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use ui::fonts::{Fonts, SOURCECODE_REGULAR_FONT};

use super::build::{HudCommunicationState, CONSOLE_FONT_COLOR, MESSAGES_DEFAULT_MAX_WIDTH};

pub fn console_input(
    mut events: EventReader<ClientSideConsoleInput>,
    mut net: EventWriter<OutgoingReliableClientMessage<ConsoleCommandsClientMessage>>,
    mut display: EventWriter<DisplayConsoleMessage>,
    fonts: Res<Fonts>,
) {
    for input in events.iter() {
        let source = fonts.handles.get(SOURCECODE_REGULAR_FONT).unwrap();

        let section = TextSection::new(
            input.to_string(),
            TextStyle {
                font: source.clone(),
                font_size: 12.0,
                color: CONSOLE_FONT_COLOR,
            },
        );

        display.send(DisplayConsoleMessage {
            sections: vec![section],
        });

        net.send(OutgoingReliableClientMessage {
            message: ConsoleCommandsClientMessage::ConsoleCommand(input.clone()),
        });
    }
}

pub(crate) fn receive_console_message(
    mut net: EventReader<IncomingReliableServerMessage<ConsoleCommandsServerMessage>>,
    fonts: Res<Fonts>,
    mut events: EventWriter<DisplayConsoleMessage>,
) {
    for message in net.iter() {
        match &message.message {
            ConsoleCommandsServerMessage::ConsoleWriteLine(message) => {
                let mut sections = vec![];

                for net_section in message.sections.iter() {
                    sections.push(TextSection::new(
                        net_section.text.clone(),
                        TextStyle {
                            font: fonts
                                .handles
                                .get(fonts.map.get(&net_section.font).expect("Font not loaded"))
                                .unwrap()
                                .clone(),
                            font_size: net_section.font_size,
                            color: net_section.color,
                        },
                    ));
                }

                events.send(DisplayConsoleMessage { sections });
            }
            _ => (),
        }
    }
}
#[derive(Event)]
pub struct DisplayConsoleMessage {
    pub sections: Vec<TextSection>,
}

#[derive(Component)]
pub struct CommunicationTextBundle;

pub(crate) fn display_console_message(
    mut events: EventReader<DisplayConsoleMessage>,
    mut commands: Commands,
    chat_state: Res<HudCommunicationState>,
) {
    for event in events.iter() {
        let mut sections = event.sections.clone();
        let first = sections.first_mut().unwrap();
        first.value = "> ".to_string() + &first.value;

        let text_section = commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Auto,
                    height: Val::Auto,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert((Label, AccessibilityNode(NodeBuilder::new(Role::ListItem))))
            .with_children(|parent| {
                parent
                    .spawn(
                        TextBundle::from_sections(sections.clone()).with_style(Style {
                            max_width: Val::Px(MESSAGES_DEFAULT_MAX_WIDTH),
                            max_height: Val::Auto,
                            ..Default::default()
                        }),
                    )
                    .insert(CommunicationTextBundle);
            })
            .id();

        commands
            .entity(chat_state.console_messages_node)
            .insert_children(0, &[text_section]);
    }
}
