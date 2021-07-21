use std::collections::HashMap;

use bevy::prelude::{EventReader, EventWriter, Local, Query};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::{general::rcon_authorization::RconAuthorization, net::net_rcon_authorization::NetRconAuthorization}, structs::network_messages::ReliableServerMessage};

const RCON_PASSWORD  : &str = "KA-BAR";


#[derive(Default)]
pub struct BruteforceProtection {

    pub tracking_data : HashMap<u32, u8>,
    pub blacklist : Vec<u32>,

}

pub fn rcon_authorization_event(
    mut authorization_events : EventReader<RconAuthorization>,
    mut bruteforce_protection : Local<BruteforceProtection>,
    mut connected_players : Query<&mut ConnectedPlayer>,
    mut net_rcon_authorization : EventWriter<NetRconAuthorization>,
) {

    for authorization_event in authorization_events.iter() {


        if bruteforce_protection.blacklist.contains(&authorization_event.handle) {

            net_rcon_authorization.send(NetRconAuthorization {
                handle: authorization_event.handle,
                message: ReliableServerMessage::ConsoleWriteLine(
                    "[color=#ff0000]Too many past attempts, blacklisted.[/color]"
                    .to_string()
                ),
            });
            continue;
            
        }

        if authorization_event.input_password == RCON_PASSWORD {

            let mut connected_player_component = connected_players.get_mut(authorization_event.entity).unwrap();

            connected_player_component.rcon = true;

            net_rcon_authorization.send(NetRconAuthorization {
                handle: authorization_event.handle,
                message: ReliableServerMessage::ConsoleWriteLine(
                    "[color=#3cff00]RCON status granted![/color]"
                    .to_string()
                ),
            });


        } else {
            match bruteforce_protection.tracking_data.get_mut(&authorization_event.handle) {
                Some(attempt_amount) => {
                    *attempt_amount+=1;
                    if attempt_amount > &mut 10 {
                        bruteforce_protection.blacklist.push(authorization_event.handle);
                    }
                },
                None => {
                    bruteforce_protection.tracking_data.insert(authorization_event.handle, 1);
                },
            }

            net_rcon_authorization.send(NetRconAuthorization {
                handle: authorization_event.handle,
                message: ReliableServerMessage::ConsoleWriteLine(
                    "[color=#ff6600]Wrong password.[/color]"
                    .to_string()
                ),
            });

        }
    }

}
