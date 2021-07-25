use crate::space_core::{resources::{network_messages::GridMapType, precalculated_fov_data::Vec3Int}};

pub struct ShipCell {
    pub item : i64,
    pub id : Vec3Int,
    pub grid_type : GridMapType,
}
