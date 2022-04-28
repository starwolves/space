use bevy_core::Timer;
use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct LongLineArrow;

#[derive(Component)]
pub struct PointArrow {
    pub timer: Timer,
}
