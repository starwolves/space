use crate::space_core::{resources::{doryen_fov::Vec3Int, network_messages::GridMapType}};

pub struct ShipCell {
    pub item : i64,
    pub id : Vec3Int,
    pub grid_type : GridMapType,
}
