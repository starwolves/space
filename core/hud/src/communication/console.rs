use bevy::{
    prelude::{
        AssetServer, BuildChildren, Commands, EventReader, EventWriter, NodeBundle, Res, TextBundle,
    },
    text::{TextSection, TextStyle},
    ui::{FlexDirection, Size, Style, Val},
};
use console_commands::net::{
    ClientConsoleInput, ConsoleCommandsClientMessage, ConsoleCommandsServerMessage,
};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use ui::fonts::{Fonts, SOURCECODE_REGULAR_FONT};

use super::build::{HudCommunicationState, CONSOLE_FONT_COLOR};

pub(crate) fn console_input(
    mut events: EventReader<ClientConsoleInput>,
    mut net: EventWriter<OutgoingReliableClientMessage<ConsoleCommandsClientMessage>>,
    mut display: EventWriter<DisplayConsoleMessage>,
    asset_server: Res<AssetServer>,
) {
    for input in events.iter() {
        let source = asset_server.load(SOURCECODE_REGULAR_FONT);

        let section = TextSection::new(
            input.to_string(),
            TextStyle {
                font: source,
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
    asset_server: Res<AssetServer>,
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
                            font: asset_server
                                .load(fonts.map.get(&net_section.font).expect("Font not loaded")),
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

pub struct DisplayConsoleMessage {
    pub sections: Vec<TextSection>,
}

pub(crate) fn display_console_message(
    mut events: EventReader<DisplayConsoleMessage>,
    mut commands: Commands,
    chat_state: Res<HudCommunicationState>,
) {
    for event in events.iter() {
        let text_section = commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                    flex_direction: FlexDirection::ColumnReverse,

                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_sections(event.sections.clone()));
            })
            .id();

        commands
            .entity(chat_state.console_messages_node)
            .insert_children(0, &[text_section]);
    }
}
