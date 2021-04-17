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
    GeneratedName(String)
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
        }

    }


}
