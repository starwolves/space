use bevy::prelude::{EventReader, ResMut, warn};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::events::net::{net_chat_message::NetChatMessage, net_done_boarding::NetDoneBoarding, net_drop_current_item::NetDropCurrentItem, net_load_entity::NetLoadEntity, net_on_boarding::NetOnBoarding, net_on_new_player_connection::NetOnNewPlayerConnection, net_on_setupui::NetOnSetupUI, net_on_spawning::NetOnSpawning, net_pickup_world_item::NetPickupWorldItem, net_rcon_authorization::NetRconAuthorization, net_send_entity_updates::NetSendEntityUpdates, net_send_world_environment::NetSendWorldEnvironment, net_showcase::NetShowcase, net_switch_hands::NetSwitchHands, net_takeoff_item::NetTakeOffItem, net_unload_entity::NetUnloadEntity, net_wear_item::NetWearItem};


pub fn net_send_messages_event(
    tuple0 : (
        ResMut<NetworkResource>,
        EventReader<NetOnBoarding>,
        EventReader<NetOnNewPlayerConnection>,
        EventReader<NetOnSetupUI>,
        EventReader<NetDoneBoarding>,
        EventReader<NetLoadEntity>,
        EventReader<NetUnloadEntity>,
        EventReader<NetSendEntityUpdates>,
        EventReader<NetSendWorldEnvironment>,
        EventReader<NetChatMessage>,
        EventReader<NetOnSpawning>,
        EventReader<NetPickupWorldItem>,
        EventReader<NetDropCurrentItem>,
        EventReader<NetSwitchHands>,
        EventReader<NetWearItem>,
        EventReader<NetTakeOffItem>,
    ),
    tuple1 : (
        EventReader<NetShowcase>,
        EventReader<NetRconAuthorization>,
    )
) {


    let (
        mut net,
        mut net_on_boarding,
        mut net_on_new_player_connection,
        mut net_on_setupui,
        mut net_done_boarding,
        mut net_load_entity,
        mut net_unload_entity,
        mut net_send_entity_updates,
        mut net_send_world_environment,
        mut net_chat_message,
        mut net_on_spawning,
        mut net_pickup_world_item,
        mut net_drop_current_item,
        mut net_switch_hands,
        mut net_wear_item,
        mut net_takeoff_item,
    )
    = tuple0;
    
    let (
        mut net_showcase,
        mut net_rcon_authorization
    )
    = tuple1;






    for new_event in net_on_spawning.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_on_spawning message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_spawning message (1): {:?}", err);
            }
        };

    }

    for new_event in net_on_boarding.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_on_boarding message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_boarding message (1): {:?}", err);
            }
        };

    }

    for new_event in net_on_new_player_connection.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_on_new_player_connection message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_new_player_connection message (1): {:?}", err);
            }
        };

    }

    for new_event in net_on_setupui.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_on_setupui message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_setupui message (1): {:?}", err);
            }
        };

    }

    for new_event in net_done_boarding.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_done_boarding message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_done_boarding message (1): {:?}", err);
            }
        };

    }

    for new_event in net_unload_entity.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_unload_entity message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_unload_entity message (1): {:?}", err);
            }
        };

    }

    for new_event in net_load_entity.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_load_entity message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_load_entity message (1): {:?}", err);
            }
        };

    }

    for new_event in net_send_entity_updates.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_send_entity_updates message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_send_entity_updates message (1): {:?}", err);
            }
        };

    }

    for new_event in net_send_world_environment.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_send_world_environment message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_send_world_environment message (1): {:?}", err);
            }
        };

    }

    for new_event in net_chat_message.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_chat_message message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_chat_message message (1): {:?}", err);
            }
        };

    }

    for new_event in net_pickup_world_item.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_pickup_world_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_pickup_world_item message (1): {:?}", err);
            }
        };

    }

    for new_event in net_drop_current_item.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_drop_current_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_drop_current_item message (1): {:?}", err);
            }
        };

    }

    for new_event in net_switch_hands.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_switch_hands message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_switch_hands message (1): {:?}", err);
            }
        };

    }

    for new_event in net_wear_item.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_wear_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_wear_item message (1): {:?}", err);
            }
        };

    }

    for new_event in net_takeoff_item.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_takeoff_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_takeoff_item message (1): {:?}", err);
            }
        };

    }
    

    for new_event in net_showcase.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_showcase message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_showcase message (1): {:?}", err);
            }
        };

    }
    
    for new_event in net_rcon_authorization.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_rcon_authorization message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_rcon_authorization message (1): {:?}", err);
            }
        };

    }
    


}
