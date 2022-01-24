use bevy::prelude::{Entity, Component};

#[derive(Component)]
pub struct RigidBodyLinkTransform {

    pub follow_entity : Entity,
    pub active : bool,

}
impl Default for RigidBodyLinkTransform {
    fn default() -> Self {
        Self {
            follow_entity : Entity::new(0),
            active : true,
        }
    }
}
