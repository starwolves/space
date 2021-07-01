use crate::space_core::{resources::precalculated_fov_data::Vec3Int, structs::network_messages::GridMapType};

pub struct ExamineMap{
    pub handle : u32,
    pub gridmap_type : GridMapType,
    pub gridmap_cell_id : Vec3Int,
}
