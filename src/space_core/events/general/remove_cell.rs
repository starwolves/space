use crate::space_core::resources::{doryen_fov::Vec3Int, network_messages::GridMapType, gridmap_main::CellData};

pub struct RemoveCell {
    pub handle : u32,
    pub gridmap_type : GridMapType,
    pub id: Vec3Int,
    pub cell_data : CellData,
}
