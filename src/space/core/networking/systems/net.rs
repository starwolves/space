use bevy::prelude::{EventReader, ResMut, warn, Query};
use bevy_networking_turbulence::NetworkResource;

use crate::space::{entities::construction_tool_admin::events::NetConstructionTool, core::{pawn::{components::ConnectedPlayer, events::{NetOnBoarding, NetOnNewPlayerConnection, NetOnSetupUI, NetDoneBoarding, NetChatMessage, NetConsoleCommands, NetUserName, NetUIInputTransmitData, NetExamineEntity, NetTabData, NetOnSpawning, NetSendWorldEnvironment, NetSendServerTime, NetUpdatePlayerCount}}, inventory::events::{NetPickupWorldItem, NetDropCurrentItem, NetSwitchHands, NetWearItem, NetTakeOffItem, NetThrowItem}, health::events::NetHealthUpdate, gridmap::events::{NetGridmapUpdates, NetProjectileFOV}, entity::events::{NetLoadEntity, NetUnloadEntity, NetShowcase, NetSendEntityUpdates}, map::events::{NetRequestDisplayModes, NetDisplayAtmospherics}}};

pub fn net_send_message_event(
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
        EventReader<NetConsoleCommands>,
        EventReader<NetUserName>,
        EventReader<NetUIInputTransmitData>,
        EventReader<NetHealthUpdate>,
        EventReader<NetExamineEntity>,
        EventReader<NetProjectileFOV>,
        EventReader<NetThrowItem>,
        EventReader<NetTabData>,
        EventReader<NetSendServerTime>,
        EventReader<NetUpdatePlayerCount>,
        EventReader<NetConstructionTool>,
        EventReader<NetGridmapUpdates>,
        EventReader<NetRequestDisplayModes>,
        EventReader<NetDisplayAtmospherics>,
    ),
    connected_players : Query<&ConnectedPlayer>,
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
        mut net_console_commands,
        mut net_user_name,
        mut net_ui_input_transmit_data,
        mut net_health_update,
        mut net_examine_entity,
        mut net_projectile_fov,
        mut net_throw_item,
        mut net_tab_data,
        mut net_send_server_time,
        mut net_update_player_count,
        mut net_construction_tool,
        mut net_gridmap_updates,
        mut net_request_display_modes,
        mut net_display_atmospherics,
    )
    = tuple1;

    let mut not_connected_handles = vec![];

    for connected_player_component in connected_players.iter() {
        if connected_player_component.connected == false {
            not_connected_handles.push(connected_player_component.handle);
        }
    }


    for new_event in net_on_spawning.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unNetRequestDisplayModesable to send net_on_spawning message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_spawning message (1): {:?}", err);
            }
        };

    }

    for new_event in net_on_boarding.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

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

    for new_event in net_console_commands.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_console_commands message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_console_commands message (1): {:?}", err);
            }
        };

    }

    for new_event in net_user_name.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_user_name message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_user_name message (1): {:?}", err);
            }
        };

    }

    for new_event in net_ui_input_transmit_data.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_ui_input_transmit_data message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_ui_input_transmit_data message (1): {:?}", err);
            }
        };

    }

    for new_event in net_health_update.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_health_update message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_health_update message (1): {:?}", err);
            }
        };

    }

    for new_event in net_examine_entity.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_examine_entity message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_examine_entity message (1): {:?}", err);
            }
        };

    }

    for new_event in net_projectile_fov.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_projectile_fov message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_projectile_fov message (1): {:?}", err);
            }
        };

    }

    for new_event in net_throw_item.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_throw_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_throw_item message (1): {:?}", err);
            }
        };

    }

    for new_event in net_tab_data.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_tab_data message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_tab_data message (1): {:?}", err);
            }
        };

    }
    
    
    for new_event in net_send_server_time.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_send_server_time message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_send_server_time message (1): {:?}", err);
            }
        };

    }


    for new_event in net_update_player_count.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_update_player_count message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_update_player_count message (1): {:?}", err);
            }
        };

    }
    

    for new_event in net_construction_tool.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_construction_tool message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_construction_tool message (1): {:?}", err);
            }
        };

    }

    for new_event in net_gridmap_updates.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_gridmap_updates message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_gridmap_updates message (1): {:?}", err);
            }
        };

    }
    
    
    for new_event in net_request_display_modes.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_request_display_modes message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_request_display_modes message (1): {:?}", err);
            }
        };

    }
    
    for new_event in net_display_atmospherics.iter() {

        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }


        match &new_event.message {
            crate::space::core::networking::resources::NetMessageType::Reliable(m) => {
                match net.send_message(new_event.handle, m.clone()) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("net_send_message_event.rs was unable to send net_display_atmospherics message: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("net_send_message_event.rs was unable to send net_display_atmospherics message (1): {:?}", err);
                    }
                };
            },
            crate::space::core::networking::resources::NetMessageType::Unreliable(m) => {
                match net.send_message(new_event.handle, m.clone()) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("net_send_message_event.rs was unable to send net_display_atmospherics message: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("net_send_message_event.rs was unable to send net_display_atmospherics message (1): {:?}", err);
                    }
                };
            },
        }

        

    }
    

}
