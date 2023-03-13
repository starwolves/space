use bevy::{
    prelude::{
        AssetServer, BuildChildren, Commands, EventReader, EventWriter, NodeBundle, Res, TextBundle,
    },
    text::{TextSection, TextStyle},
    ui::{Size, Style, Val},
};
use console_commands::net::{
    ClientConsoleInput, ConsoleCommandsClientMessage, ConsoleCommandsServerMessage,
};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use ui::fonts::Fonts;

use super::build::HudCommunicationState;

pub(crate) fn console_input(
    mut events: EventReader<ClientConsoleInput>,
    mut net: EventWriter<OutgoingReliableClientMessage<ConsoleCommandsClientMessage>>,
) {
    for input in events.iter() {
        net.send(OutgoingReliableClientMessage {
            message: ConsoleCommandsClientMessage::ConsoleCommand(input.clone()),
        });
    }
}

pub(crate) fn console_message(
    mut net: EventReader<IncomingReliableServerMessage<ConsoleCommandsServerMessage>>,
    fonts: Res<Fonts>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    chat_state: Res<HudCommunicationState>,
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
                    .entity(chat_state.console_messages_node)
                    .insert_children(0, &[text_section]);
            }
            _ => (),
        }
    }
}
