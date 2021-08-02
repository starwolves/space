use bevy::prelude::Entity;

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
