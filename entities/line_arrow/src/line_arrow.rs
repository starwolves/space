use bevy::{prelude::Component, time::Timer};

/// The component.
#[derive(Component)]
pub struct LineArrow;

/// For pointing arrows.
#[derive(Component)]
pub struct PointArrow {
    /// Timer after which the point arrow despawns.
    pub timer: Timer,
}
