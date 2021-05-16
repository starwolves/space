use bevy_rapier3d::rapier::geometry::ColliderHandle;

use crate::space_core::components::entity_data::EntityGroup;

pub struct AirLockCollision {
    pub collider1_handle : ColliderHandle,
    pub collider2_handle : ColliderHandle,
    
    pub collider1_group : EntityGroup,
    pub collider2_group : EntityGroup,

    pub intersecting : bool
}
