use bevy::{
    prelude::{EventReader, Res, ResMut, Resource},
    time::Time,
};
use bevy_xpbd_3d::prelude::{Physics, PhysicsTime};
use resources::core::TickRate;
use serde::{Deserialize, Serialize};

use crate::{client::IncomingReliableServerMessage, server::NetworkingServerMessage};

#[derive(Resource, Default, Serialize, Deserialize, Debug, Clone)]
pub struct TickRateStamp {
    pub tick: u8,
    pub iteration: u64,
    pub large: u64,
}

impl TickRateStamp {
    pub fn new(large: u64) -> Self {
        let iteration = large / u8::MAX as u64;
        let tick = (large - (iteration * u8::MAX as u64)) as u8;
        Self {
            tick,
            iteration,
            large,
        }
    }
    pub fn step(&mut self) {
        self.large += 1;
        if self.tick == u8::MAX {
            self.tick = 0;
            self.iteration += 1;
        } else {
            self.tick += 1;
        }
    }
    pub fn step_custom(&mut self, step_amount: u8) {
        self.large += step_amount as u64;
        if self.tick > u8::MAX - step_amount {
            let remainder = u8::MAX - self.tick;
            self.tick = step_amount - remainder;
            self.iteration += 1;
        } else {
            self.tick += step_amount;
        }
    }
    pub fn get_difference(&self, input: u8) -> i8 {
        let d;
        let rate = TickRate::default().fixed_rate;
        if input > self.tick {
            if u8::MAX - input < rate && input - self.tick > rate {
                d = (-((u8::MAX - input) as i16 + self.tick as i16)) as i8;
            } else {
                d = (input as i16 - self.tick as i16) as i8;
            }
        } else {
            if u8::MAX - self.tick < rate && self.tick - input > rate {
                d = ((u8::MAX as i16 - self.tick as i16) + input as i16) as i8;
            } else {
                d = (-(self.tick as i16 - input as i16)) as i8;
            }
        }
        d
    }
    pub fn calculate_large(&self, input: u8) -> u64 {
        let mut d = self.get_difference(input);
        if d > 0 {
            self.large + d as u64
        } else {
            if d.abs() as u64 > self.large {
                d = -(self.large as i8);
            }
            self.large - (d.abs() as u64)
        }
    }
}

pub(crate) fn setup_client_tickrate_stamp(
    mut client: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut stamp: ResMut<TickRateStamp>,
) {
    for event in client.read() {
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
#[derive(Resource, Default)]
pub struct PauseTickStep(pub bool);

pub fn step_tickrate_stamp(
    mut stamp: ResMut<TickRateStamp>,
    physics_loop: Res<Time<Physics>>,
    p: Res<PauseTickStep>,
) {
    if !physics_loop.is_paused() && !p.0 {
        stamp.step();
    }
}
