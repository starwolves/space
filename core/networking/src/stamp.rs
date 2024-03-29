use bevy::{
    prelude::{Res, ResMut, Resource},
    time::Time,
};
use bevy_xpbd_3d::prelude::{Physics, PhysicsTime};
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Serialize, Deserialize, Debug, Clone)]
pub struct TickRateStamp {
    pub tick: u32,
    //pub iteration: u64,
    //pub large: u64,
}
impl TickRateStamp {
    pub fn new(tick: u32) -> Self {
        //let iteration = large / (u8::MAX as u64 + 1);
        //let tick = (large - (iteration * (u8::MAX as u64 + 1))) as u8;
        Self { tick }
    }
    pub fn step(&mut self) {
        self.tick += 1;
    }
    pub fn step_custom(&mut self, step_amount: u32) {
        self.tick += step_amount;
    }
    pub fn get_difference(&self, input: u32) -> i32 {
        if input > self.tick {
            input as i32 - self.tick as i32
        } else {
            -(self.tick as i32 - input as i32)
        }
    }
}
/*
impl TickRateStamp {
    pub fn new(large: u64) -> Self {
        let iteration = large / (u8::MAX as u64 + 1);
        let tick = (large - (iteration * (u8::MAX as u64 + 1))) as u8;
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
    /// Clients should also update ClientLatency resource.
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
    pub fn get_difference(&self, input: u8) -> i16 {
        let d;
        let rate = TickRate::default().fixed_rate;
        if input > self.tick {
            if u8::MAX - input < rate && input - self.tick > rate {
                d = -((u8::MAX - input) as i16 + self.tick as i16);
            } else {
                d = input as i16 - self.tick as i16;
            }
        } else {
            if u8::MAX - self.tick < rate && self.tick - input > rate {
                d = (u8::MAX as i16 - self.tick as i16) + input as i16;
            } else {
                d = -(self.tick as i16 - input as i16);
            }
        }
        d
    }
    pub fn calculate_large(&self, input: u8) -> u64 {
        let d = self.get_difference(input);
        if d > 0 {
            self.large + d as u64
        } else {
            if d.abs() as u64 > self.large {
                0
            } else {
                self.large - (d.abs() as u64)
            }
        }
    }
}
*/
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
