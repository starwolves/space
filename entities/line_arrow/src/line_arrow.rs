use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct LineArrow;

#[derive(Component)]
pub struct PointArrow {
    pub timer: Timer,
}
