use bevy::prelude::{EventReader, ResMut, Resource};
use serde::{Deserialize, Serialize};

use crate::{client::IncomingReliableServerMessage, server::NetworkingServerMessage};

#[derive(Resource, Default, Serialize, Deserialize, Debug, Clone)]
pub struct TickRateStamp {
    pub stamp: u8,
    pub iteration: u64,
    pub large: u64,
}

impl TickRateStamp {
    pub fn step(&mut self) {
        self.large += 1;
        if self.stamp == u8::MAX {
            self.stamp = 0;
            self.iteration += 1;
        } else {
            self.stamp += 1;
        }
    }
    pub fn step_custom(&mut self, step_amount: u8) {
        self.large += step_amount as u64;
        if self.stamp > u8::MAX - step_amount {
            let remainder = u8::MAX - self.stamp;
            self.stamp = step_amount - remainder;
            self.iteration += 1;
        } else {
            self.stamp += step_amount;
        }
    }
}

pub(crate) fn setup_client_tickrate_stamp(
    mut client: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut stamp: ResMut<TickRateStamp>,
) {
    for event in client.iter() {
        match &event.message {
            NetworkingServerMessage::Awoo(sync) => {
                let mut m_stamp = sync.stamp.clone();
                m_stamp.step_custom(5);
                *stamp = m_stamp;
            }
        }
    }
}

pub fn step_tickrate_stamp(mut stamp: ResMut<TickRateStamp>) {
    stamp.step();
}
