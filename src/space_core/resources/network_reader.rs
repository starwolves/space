use bevy::prelude::EventReader;
use bevy_networking_turbulence::{NetworkEvent};

#[derive(Default)]
pub struct NetworkReader {
    pub network_events: EventReader<NetworkEvent>,
}
