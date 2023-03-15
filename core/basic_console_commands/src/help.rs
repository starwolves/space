use bevy::{
    prelude::{AssetServer, Color, EventReader, EventWriter, Res, ResMut},
    text::{TextSection, TextStyle},
};
use console_commands::{
    commands::{AllConsoleCommands, ConsoleCommand},
    net::ClientSideConsoleInput,
};
use hud::communication::console::DisplayConsoleMessage;
use ui::{fonts::SOURCECODE_REGULAR_FONT, text::COMMUNICATION_FONT_SIZE};

pub(crate) fn add_help_command(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push(ConsoleCommand {
        base: "help".to_string(),
        description: "Lists all the available commands.".to_string(),
        args: vec![],
    });
}
pub(crate) fn help_command(
    mut queue: EventReader<ClientSideConsoleInput>,
    mut console: EventWriter<DisplayConsoleMessage>,
    commands: Res<AllConsoleCommands>,
    asset_server: Res<AssetServer>,
) {
    for input in queue.iter() {
        if input.command != "help" {
            continue;
        }
        let mut console_message = "For more information about a specific command write help [command].\nAvailable console commands:\n".to_string();

        if input.args.len() == 1 {
            let arg = input.args.get(0).unwrap();
            let info_command = arg.clone();

            for command in commands.list.iter() {
                if command.base == info_command {
                    console_message.push_str(
                        &(command.description.clone() + "\n Usage: " + &command.base + " "),
                    );
                    for arg in command.args.iter() {
                        console_message.push_str(&format!("{}({:?}) ", arg.0, arg.1));
                    }
                }
            }
        } else {
            for command in commands.list.iter() {
                console_message += &format!("{}: {} \n\n", command.base, command.description);
            }
        }

        console.send(DisplayConsoleMessage {
            sections: vec![TextSection {
                value: console_message,
                style: TextStyle {
                    font: asset_server.load(SOURCECODE_REGULAR_FONT),
                    font_size: COMMUNICATION_FONT_SIZE,
                    color: Color::WHITE,
                    ..Default::default()
                },
            }],
        });
    }
}
