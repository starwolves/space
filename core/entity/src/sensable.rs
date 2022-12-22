use bevy::ecs::entity::Entity;
use bevy::prelude::Component;

/// The component for entities that can be sensed.
#[derive(Component, Default)]
#[cfg(feature = "server")]
pub struct Sensable {
    pub is_light: bool,
    pub is_audible: bool,
    pub sensed_by: Vec<Entity>,
    pub always_sensed: bool,
}
