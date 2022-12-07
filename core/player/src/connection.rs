use bevy::prelude::Component;
use bevy::prelude::Resource;

/// The component for entities int he boarding phase.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct SetupPhase;

/// The component for entities that are done boarding and about to spawn in on the ship. A stage after [Boarding].
#[derive(Component)]
#[cfg(feature = "server")]
pub struct OnBoard;

/// Event for sending server configuration to client.
#[cfg(feature = "server")]
pub struct SendServerConfiguration {
    pub handle: u64,
}
/// Resource with the current incremented authentication ID.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub(crate) struct AuthidI {
    pub i: u16,
}
