//! General physics logic.

/// Broadcast unreliable transforms to clients.
mod broadcast_interpolation_transforms;
/// Check if entities leave boundaries of physics space.
mod out_of_bounds_teleportation;
/// Physics resources.
pub mod physics;
pub mod plugin;
/// Rigid body resources.
pub mod rigid_body;
/// Link rigid body transforms.
mod rigidbody_link_transform;
/// Base rigid body spawner.
pub mod spawn_rigidbody;
