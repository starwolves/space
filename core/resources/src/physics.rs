use bevy::ecs::schedule::SystemSet;

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PhysicsSet {
    Correct,
    Cache,
}
