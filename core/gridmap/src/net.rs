use bevy::prelude::Vec3;
use networking::server::GridMapLayer;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GridmapClientMessage {
    ExamineMap(GridMapLayer, i16, i16, i16),
}
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GridmapServerMessage {
    RemoveCell(i16, i16, i16, GridMapLayer),
    AddCell(i16, i16, i16, i64, i64, GridMapLayer),
    FireProjectile(ProjectileData),
    ConfigBlackCellID(i64, i64),
    ConfigOrderedCellsMain(Vec<String>),
    ConfigOrderedCellsDetails1(Vec<String>),
    ConfigPlaceableItemsSurfaces(Vec<i64>),
    ConfigNonBlockingCells(Vec<i64>),
}

/// Contains information about the projectile and its visual graphics.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ProjectileData {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}
