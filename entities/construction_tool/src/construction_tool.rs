use bevy::prelude::{Component, Entity, Event};
use bevy_renet::renet::ClientId;
use gridmap::grid::CellIds;
use resources::grid::TargetCell;

/// The component.
#[derive(Component, Default)]

pub struct ConstructionTool {
    /// Currently selected construction cell option.
    pub construction_option: Option<CellIds>,
}

/// Player requested input event.
#[derive(Event)]
pub struct InputConstruct {
    /// Connection handle that fired this input.
    pub handle_option: Option<ClientId>,
    /// Build on gridmap cell:
    pub target_cell: TargetCell,
    /// Entity that requested to construct.
    pub belonging_entity: Entity,
}
/// Player requested input event.
#[derive(Event)]
pub struct InputConstructionOptions {
    /// Connection handle that fired this input.
    pub handle_option: Option<ClientId>,
    /// Entity that requested to select construction option.
    pub entity: Entity,
}
/// Player requested input event.
#[derive(Event)]
pub struct InputDeconstruct {
    /// Connection handle that fired this input.
    pub handle_option: Option<ClientId>,
    pub target_cell_option: Option<TargetCell>,
    pub target_entity_option: Option<Entity>,
    /// Entity that requested to deconstruct.
    pub belonging_entity: Entity,
}
