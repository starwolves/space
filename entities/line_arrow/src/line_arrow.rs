use bevy::{prelude::Component, time::Timer};

/// The component for line arrows.
#[derive(Component)]
pub struct LineArrow;

/// The component for pointing arrows.
#[derive(Component)]
pub struct PointArrow {
    /// Timer after which the point arrow despawns.
    pub timer: Timer,
}
