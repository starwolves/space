use crate::space_core::resources::{doryen_fov::Vec3Int, network_messages::GridMapType};

pub struct RemoveCell {
    pub handle : u32,
    pub gridmap_type : GridMapType,
    pub id: Vec3Int,
}
