//! General physics logic.

/// Cache physics data of previous ticks.
pub mod cache;
pub mod correction_mode;
/// Decoupled phyisics entities.
pub mod entity;
/// Link rigid body transforms.
pub mod mirror_physics_transform;
pub mod net;
/// Physics resources.
pub mod physics;
pub mod plugin;
/// Rigid body resources.
pub mod rigid_body;
/// Base rigid body spawner.
pub mod spawn;
/// Networking syncing.
pub mod sync;
