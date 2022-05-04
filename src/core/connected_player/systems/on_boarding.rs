use bevy_ecs::{event::EventWriter, prelude::Added, system::Query};

use crate::core::{
    connected_player::{
        components::{Boarding, ConnectedPlayer},
        events::NetOnBoarding,
    },
    networking::resources::ReliableServerMessage,
};

use super::on_setupui::INPUT_NAME_PATH;

pub fn on_boarding(
    query: Query<&ConnectedPlayer, Added<Boarding>>,
    mut net_on_boarding: EventWriter<NetOnBoarding>,
) {
    for connected_player_component in query.iter() {
        net_on_boarding.send(NetOnBoarding {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::UIRequestInput(
                "setupUI".to_string(),
                INPUT_NAME_PATH.to_string(),
            ),
        });
    }
}
