use crate::core::networking::resources::{GridMapType, NetProjectileType, ReliableServerMessage};

use super::resources::{CellData, Vec3Int};

pub struct RemoveCell {
    pub handle_option: Option<u64>,
    pub gridmap_type: GridMapType,
    pub id: Vec3Int,
    pub cell_data: CellData,
}

pub struct NetGridmapUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct ProjectileFOV {
    pub laser_projectile: NetProjectileType,
}

pub struct NetProjectileFOV {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
