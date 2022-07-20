use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct RigidBodyLinkTransform {
    pub follow_entity: Entity,
    pub active: bool,
}
impl Default for RigidBodyLinkTransform {
    fn default() -> Self {
        Self {
            follow_entity: Entity::from_raw(0),
            active: true,
        }
    }
}
