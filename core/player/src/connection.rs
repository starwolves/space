use bevy::prelude::Component;
use bevy::prelude::Resource;
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;

#[cfg(feature = "server")]
#[derive(NetMessage)]
pub(crate) struct NetPlayerConn {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// The component for players that are requesting boarding.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Boarding;

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