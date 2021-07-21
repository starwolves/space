use bevy::prelude::EventReader;

use crate::space_core::events::general::rcon_authorization::RconAuthorization;

pub fn rcon_authorization_event(
    mut authorization_events : EventReader<RconAuthorization>
) {

    for authorization_event in authorization_events.iter() {


        


    }

}
