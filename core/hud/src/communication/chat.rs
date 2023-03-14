use bevy::{
    prelude::{AssetServer, BuildChildren, Commands, EventReader, NodeBundle, Res, TextBundle},
    text::{TextSection, TextStyle},
    ui::{FlexDirection, Size, Style, Val},
};
use chat::net::ChatServerMessage;
use networking::client::IncomingReliableServerMessage;
use ui::fonts::Fonts;

use super::build::HudCommunicationState;

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
                            flex_direction: FlexDirection::ColumnReverse,

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
