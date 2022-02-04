
use crate::space_core::ecs::networking::resources::{ReliableServerMessage, NetProjectileType, GridMapType};

use super::resources::{CellData, Vec3Int};

pub struct RemoveCell {
    pub handle : u32,
    pub gridmap_type : GridMapType,
    pub id: Vec3Int,
    pub cell_data : CellData,
}

pub struct NetGridmapUpdates {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct ProjectileFOV {
    pub laser_projectile : NetProjectileType,
}

pub struct NetProjectileFOV {
    pub handle : u32,
    pub message : ReliableServerMessage
}
