use bevy::prelude::Vec3;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::grid::Orientation;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum GridmapClientMessage {
    ExamineMap(i16, i16, i16),
}
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum GridmapServerMessage {
    RemoveCell(i16, i16, i16),
    AddCell(i16, i16, i16, u16, Option<Orientation>),
    FireProjectile(ProjectileData),
    ConfigBlackCellID(u16, u16),
    ConfigOrderedCellsMain(Vec<String>),
    ConfigOrderedCellsDetails1(Vec<String>),
    ConfigPlaceableItemsSurfaces(Vec<u16>),
    ConfigNonBlockingCells(Vec<u16>),
}

/// Contains information about the projectile and its visual graphics.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum ProjectileData {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}
