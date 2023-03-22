use std::{fs::File, io::Write, path::Path};

use bevy::prelude::{EventReader, EventWriter, Res, ResMut};
use console_commands::{
    commands::{AllConsoleCommands, ConsoleCommand, InputConsoleCommand},
    net::{ConsoleCommandsServerMessage, ConsoleLine},
};
use gridmap::grid::Gridmap;
use hud::communication::build::CONSOLE_FONT_COLOR;
use networking::server::OutgoingReliableServerMessage;
use ui::{
    fonts::{Fonts, SOURCECODE_REGULAR_FONT},
    text::{NetTextSection, COMMUNICATION_FONT_SIZE},
};

pub(crate) fn add_export_map_command(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push(ConsoleCommand {
        base: "exportMap".to_string(),
        description: "Exports the map to ron.".to_string(),
        args: vec![],
    });
}

pub(crate) fn export_map(
    mut queue: EventReader<InputConsoleCommand>,
    gridmap: Res<Gridmap>,
    mut net: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    fonts: Res<Fonts>,
) {
    for command in queue.iter() {
        if command.input.command == "exportMap" {
            let mut i = 0;

            loop {
                let path = Path::new("data")
                    .join("maps")
                    .join("bullseye")
                    .join(format!("main_export{}.bin", i));
                if !path.exists() {
                    let mut file = File::create(path.clone()).unwrap();
                    file.write_all(&gridmap.export_binary()).unwrap();
                    match command.handle_option {
                        Some(handle) => {
                            net.send(OutgoingReliableServerMessage {
                                message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                                    ConsoleLine {
                                        sections: vec![NetTextSection {
                                            text: format!("Exported gridmap to {:?}", path),
                                            font: *fonts
                                                .inv_map
                                                .get(SOURCECODE_REGULAR_FONT)
                                                .unwrap(),
                                            font_size: COMMUNICATION_FONT_SIZE,
                                            color: CONSOLE_FONT_COLOR,
                                        }],
                                    },
                                ),
                                handle,
                            });
                        }
                        None => {}
                    }
                    break;
                } else {
                    i += 1;
                }
            }
        }
    }
}
