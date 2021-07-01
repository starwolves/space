use crate::space_core::{resources::precalculated_fov_data::Vec3Int, structs::network_messages::GridMapType};

pub struct ShipCell {
    pub item : i64,
    pub id : Vec3Int,
    pub grid_type : GridMapType,
}
