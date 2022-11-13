use bevy::prelude::Component;

/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Jumpsuit;
#[cfg(feature = "server")]
pub const JUMPSUIT_SECURITY_ENTITY_NAME: &str = "jumpsuitSecurity";
