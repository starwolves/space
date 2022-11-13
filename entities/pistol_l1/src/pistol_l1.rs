use bevy::prelude::Component;

/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct PistolL1;
#[cfg(feature = "server")]
pub const PISTOL_L1_ENTITY_NAME: &str = "pistolL1";
