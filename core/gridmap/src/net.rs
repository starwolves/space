use bevy::prelude::Vec3;
use resources::grid::TargetCell;
use resources::math::Vec3Int;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::grid::CellIds;
use crate::grid::CellTypeId;
use crate::grid::CellTypeName;
use crate::grid::TargetCellWithOrientationWType;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum GridmapClientMessage {
    ExamineMap(i16, i16, i16),
    ConstructCells(ConstructCell),
    DeconstructCells(DeconstructCell),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstructCell {
    pub cells: Vec<TargetCellWithOrientationWType>,
    // For when using groups to pass an absolute coordinate.
    pub group_option: Option<Vec3Int>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewCell {
    pub cell: TargetCell,
    pub orientation: u8,
    pub tile_type: CellTypeId,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeconstructCell {
    pub cells: Vec<TargetCell>,
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum GridmapServerMessage {
    RemoveCell(TargetCell),
    AddCell(NewCell),
    FireProjectile(ProjectileData),
    ConfigOrderedCellsMain(Vec<CellTypeName>),
    GhostCellType(CellIds),
}

/// Contains information about the projectile and its visual graphics.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum ProjectileData {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}
