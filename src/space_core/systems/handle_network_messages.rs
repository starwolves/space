use bevy::{ecs::system::{ResMut,Res}, prelude::{info, warn}};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{
    functions::{
        name_generator
    },
    resources::{
        server_id::ServerId,
        used_names::UsedNames
    },
    structs::network_messages::{
        ReliableClientMessage, 
        ReliableServerMessage,
        UIInputAction, 
        UIInputNodeClass,
        EntityUpdateData
    }
};

struct PostProcessPerHandle {
    process : PostProcess,
    handle : u32
}

enum PostProcess {
    GeneratedName(String),
    UIRequestInput(String, String)
}

pub fn handle_network_messages(
    mut net: ResMut<NetworkResource>,
    mut used_names : ResMut<UsedNames>,
    server_id : Res<ServerId>
) {

    let mut post_processes : Vec<PostProcessPerHandle> = vec![];

    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ReliableClientMessage>() {
            info!("ReliableClientMessage received on [{}]: {:?}",handle, client_message);

            match client_message {
                ReliableClientMessage::Awoo => {},
                ReliableClientMessage::UIInput(node_class,action,node_name,ui_type) => {

                    if ui_type == "setupUI" {

                        if node_name == "board" && matches!(node_class, UIInputNodeClass::Button) && matches!(action, UIInputAction::Pressed) {

                            info!("Received boarding request from [{}]", handle);

                            post_processes.push(PostProcessPerHandle{
                                handle: *handle,
                                process: PostProcess::UIRequestInput(
                                    "setupUI".to_string(),
                                    "HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName".to_string()
                                )
                            });

                        }

                    }

                }
                ReliableClientMessage::SceneReady => {

                    let suggested_name = name_generator::get_full_name(true, true, &used_names);

                    post_processes.push(PostProcessPerHandle{
                        handle: *handle,
                        process: PostProcess::GeneratedName(suggested_name)
                    });
                    

                }
                ReliableClientMessage::UIInputTransmitText(ui_type, node_path, input_text) => {

                    if ui_type == "setupUI" {

                        if node_path == 
                        "HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName" {

                            if used_names.names.contains(&input_text) {
                                continue;
                            }

                            used_names.names.push(input_text.clone());

                            // We have the player's name, now fully spawn in the player and remove from softConnected

                            info!("{} [{}] has boarded the spaceship.",input_text, handle);

                        }

                    }

                }
            }

        }

        while let Some(_server_message) = channels.recv::<ReliableServerMessage>() {
            // In case we ever get this from faulty or malicious clients, free it up.
        }

    }

    for post_process_per_handle in post_processes.iter() {

        match &post_process_per_handle.process {
            PostProcess::GeneratedName(suggested_name) => {

                match net.send_message(post_process_per_handle.handle, ReliableServerMessage::EntityUpdate(
                    server_id.id.id(),
                    "setupUI::HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName".to_string(),
                    EntityUpdateData::UIText(suggested_name.to_string()))
                ) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("handle_network_messages.rs was unable to send suggested name: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("handle_network_messages.rs was unable to send suggested name (1): {:?}", err);
                    }
                };

            }
            PostProcess::UIRequestInput(ui_type, node_path) => {

                match net.send_message(post_process_per_handle.handle, ReliableServerMessage::UIRequestInput(
                    ui_type.to_string(),
                    node_path.to_string()
                )) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("handle_network_messages.rs was unable to UIRequestInput: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("handle_network_messages.rs was unable to UIRequestInput (1): {:?}", err);
                    }
                };

            }
        }

    }


}
