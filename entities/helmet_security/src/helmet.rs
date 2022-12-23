use bevy::prelude::Component;

/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Helmet;
#[cfg(feature = "server")]
pub const HELMET_SECURITY_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "helmetSecurity");
use const_format::concatcp;
use resources::content::SF_CONTENT_PREFIX;
