use bevy::prelude::Transform;

/// For entities that are also registered with the gridmap.
pub struct GridItemData {
    pub transform_offset: Transform,
    /// So this entity can be built on a cell when another item is already present on that cell.
    pub can_be_built_with_grid_item: Vec<String>,
}
