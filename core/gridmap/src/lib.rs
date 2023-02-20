//! The gridmap consisting of cells that represent the ship.
//! A spaceship is an enclosed area and can be dynamically modified, constructed and deconstructed by players.
//! The spaceship is split up into tiles/(ship-)cells which can either be a floor tile, a wall tile or an entity tile.
//! "Entity tiles" are when entities are statically placed on the gridmap and take up gridmap space.
//! Tiles/cells can be examined and are included with FOV calculations.
//! Sensing abilities are integrated with examinig the gridmap so players can examine additional information about individual cells.
//! The gridmap is split up into multiple layers that can both exist peacefully on the same tiles. One is a "main" layer which includes the main ship construction parts, like walls and floors and tile entities that block other wall constructions. Whereas the "details1" layer is for added details on each gridmap cell, like posters, small lights, repeated visual effects and indicators etc.
//! Not all tiles are their own traditional entity with their own components, in fact most tiles that have no expected special behaviour have their data stored in a resource rather than as an individual entity with an ID for performance reasons.

/// Check if an entity can reach another entity.
pub mod can_reach_entity;
/// Configuration to send to newly connected clients.
pub mod connections;
/// Manage gridmap exmination.
pub mod examine;
/// Manage gridmap FOV.
pub mod fov;
/// Get a spawn position for an item that is free.
pub mod get_spawn_position;
/// Client-side graphics handling, such as loading and spawning meshes and textures for gridmap items.
pub mod graphics;
/// Core gridmap resources.
pub mod grid;
/// Initialize gridmap meta data.
mod init;
pub mod net;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Manage sensing authorization for gridmap examining.
mod sensing_ability;
/// Manage gridmap updates.
pub mod updates;
pub mod wall;
