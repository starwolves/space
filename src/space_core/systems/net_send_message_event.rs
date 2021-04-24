use bevy::prelude::{EventReader, ResMut, warn};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::events::{net_done_boarding::NetDoneBoarding, net_on_boarding::NetOnBoarding, net_on_new_player_connection::NetOnNewPlayerConnection, net_on_setupui::NetOnSetupUI, net_visible_checker::NetVisibleChecker};


pub fn net_send_messages_event(
    mut net: ResMut<NetworkResource>,
    mut net_on_boarding : EventReader<NetOnBoarding>,
    mut net_on_new_player_connection : EventReader<NetOnNewPlayerConnection>,
    mut net_on_setupui : EventReader<NetOnSetupUI>,
    mut net_done_boarding : EventReader<NetDoneBoarding>,
    mut net_visible_checker : EventReader<NetVisibleChecker>
) {

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


    for new_event in net_visible_checker.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_visible_checker message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_visible_checker message (1): {:?}", err);
            }
        };

    }

    

}
