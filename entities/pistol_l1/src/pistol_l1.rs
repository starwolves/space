use bevy::prelude::Component;
use const_format::concatcp;
use entity::meta::SF_CONTENT_PREFIX;
/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct PistolL1;
#[cfg(feature = "server")]
pub const PISTOL_L1_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "pistolL1");
