use bevy::{core::Timer, prelude::Component};

#[derive(Component)]
pub struct LineArrow;

#[derive(Component)]
pub struct PointArrow {
    pub timer: Timer,
}
