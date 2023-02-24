use bevy::prelude::Vec3;
use resources::grid::TargetCell;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum GridmapClientMessage {
    ExamineMap(i16, i16, i16),
    ConstructCell(ConstructCell),
    DeconstructCell(DeconstructCell),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstructCell {
    pub cell: TargetCell,
    pub orientation: u8,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewCell {
    pub cell: TargetCell,
    pub orientation: u8,
    pub tile_type: u16,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeconstructCell {
    pub cell: TargetCell,
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum GridmapServerMessage {
    RemoveCell(TargetCell),
    AddCell(NewCell),
    FireProjectile(ProjectileData),
    ConfigBlackCellID(u16, u16),
    ConfigOrderedCellsMain(Vec<String>),
    ConfigOrderedCellsDetails1(Vec<String>),
    ConfigPlaceableItemsSurfaces(Vec<u16>),
    ConfigNonBlockingCells(Vec<u16>),
    GhostCellType(u16),
}

/// Contains information about the projectile and its visual graphics.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum ProjectileData {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}
