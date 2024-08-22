use bevy::ecs::component::Component;

// Entity with this component is controleld by the local client.
#[derive(Component)]
pub struct ClientPawn;
pub const HUMANOID_HEIGHT: f32 = 1.6;
