use bevy::{ecs::system::{ResMut}, prelude::{EventWriter, info}};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{events::general::{scene_ready::SceneReady, ui_input::UIInput, ui_input_transmit_text::UIInputTransmitText}, structs::network_messages::{
        ReliableClientMessage, 
        ReliableServerMessage
    }
};

pub fn handle_network_messages(
    mut net: ResMut<NetworkResource>,
    mut ui_input_event : EventWriter<UIInput>,
    mut scene_ready_event : EventWriter<SceneReady>,
    mut ui_input_transmit_text : EventWriter<UIInputTransmitText>
) {



    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        

        while let Some(client_message) = channels.recv::<ReliableClientMessage>() {
            info!("ReliableClientMessage received on [{}]: {:?}",handle, client_message);

            match client_message {
                ReliableClientMessage::Awoo => {},
                ReliableClientMessage::UIInput(
                    node_class,
                    action,
                    node_name,
                    ui_type
                ) => {
                    ui_input_event.send(UIInput{
                        handle : *handle,
                        node_class: node_class,
                        action: action,
                        node_name : node_name,
                        ui_type : ui_type
                    });
                }
                ReliableClientMessage::SceneReady(scene_type) => {
                    scene_ready_event.send(SceneReady{
                        handle: *handle,
                        scene_type: scene_type
                    });
                }
                ReliableClientMessage::UIInputTransmitText(ui_type, node_path, input_text) => {
                    ui_input_transmit_text.send(UIInputTransmitText{
                        handle: *handle,
                        ui_type:ui_type,
                        node_path:node_path,
                        input_text:input_text
                    });
                }

            }

        }

        while let Some(_server_message) = channels.recv::<ReliableServerMessage>() {
            // In case we ever get this from faulty or malicious clients, free it up.
        }

    }


}
