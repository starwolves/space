use bevy::prelude::{EventReader, ResMut, warn};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::events::net::{net_chat_message::NetChatMessage, net_done_boarding::NetDoneBoarding, net_drop_current_item::NetDropCurrentItem, net_load_entity::NetLoadEntity, net_on_boarding::NetOnBoarding, net_on_new_player_connection::NetOnNewPlayerConnection, net_on_setupui::NetOnSetupUI, net_on_spawning::NetOnSpawning, net_pickup_world_item::NetPickupWorldItem, net_send_entity_updates::NetSendEntityUpdates, net_send_world_environment::NetSendWorldEnvironment, net_switch_hands::NetSwitchHands, net_takeoff_item::NetTakeOffItem, net_unload_entity::NetUnloadEntity, net_wear_item::NetWearItem};


pub fn net_send_messages_event0(
    mut net: ResMut<NetworkResource>,
    mut net_on_boarding : EventReader<NetOnBoarding>,
    mut net_on_new_player_connection : EventReader<NetOnNewPlayerConnection>,
    mut net_on_setupui : EventReader<NetOnSetupUI>,
    mut net_done_boarding : EventReader<NetDoneBoarding>,
    mut net_load_entity : EventReader<NetLoadEntity>,
    mut net_unload_entity : EventReader<NetUnloadEntity>,
    mut net_send_entity_updates: EventReader<NetSendEntityUpdates>,
    mut net_send_world_environment : EventReader<NetSendWorldEnvironment>,
    mut net_chat_message : EventReader<NetChatMessage>,
    mut net_on_spawning : EventReader<NetOnSpawning>,
    mut net_pickup_world_item : EventReader<NetPickupWorldItem>,
    mut net_drop_current_item : EventReader<NetDropCurrentItem>,
    mut net_switch_hands : EventReader<NetSwitchHands>,
    mut net_wear_item : EventReader<NetWearItem>,
    mut net_takeoff_item : EventReader<NetTakeOffItem>,
) {

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
    
    
    
}
