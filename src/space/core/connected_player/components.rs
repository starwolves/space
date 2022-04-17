use bevy_ecs::prelude::Component;
use bevy_transform::components::Transform;

#[derive(Component)]
pub struct ConnectedPlayer {
    pub handle: u32,
    pub authid: u16,
    pub rcon: bool,
    pub connected: bool,
}

impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: 0,
            authid: 0,
            rcon: true,
            connected: true,
        }
    }
}

#[derive(Component)]
pub struct Boarding;

#[derive(Component)]
pub struct SoftPlayer;

#[derive(Component)]
pub struct Spawning {
    pub transform: Transform,
}

#[derive(Component)]
pub struct SetupPhase;

#[derive(Component)]
pub struct OnBoard;
