use crate::space_core::resources::network_messages::GridMapType;

pub struct InputDeconstruct {
    pub handle : u32,
    pub target_cell : (GridMapType, i16,i16,i16),
    pub belonging_entity : u64,
}
