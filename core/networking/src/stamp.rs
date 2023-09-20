use bevy::prelude::{EventReader, Res, ResMut, Resource};
use bevy_xpbd_3d::prelude::PhysicsLoop;
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
    pub fn get_difference(&self, input: u8) -> i16 {
        let d;
        if input > self.stamp {
            if u8::MAX - input < 64 && input - self.stamp > 64 {
                d = -((u8::MAX - input) as i16 + self.stamp as i16);
            } else {
                d = input as i16 - self.stamp as i16;
            }
        } else {
            if u8::MAX - self.stamp < 64 && self.stamp - input > 64 {
                d = (u8::MAX as i16 - self.stamp as i16) + input as i16;
            } else {
                d = -(self.stamp as i16 - input as i16);
            }
        }
        d
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
            _ => (),
        }
    }
}

pub fn step_tickrate_stamp(mut stamp: ResMut<TickRateStamp>, physics_loop: Res<PhysicsLoop>) {
    if !physics_loop.paused {
        stamp.step();
    }
}
