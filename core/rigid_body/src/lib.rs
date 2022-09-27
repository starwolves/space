//! Bevy Rapier 3D rigid body integration.
//! A base for rigid bodies.
//! All physics entities that aren't sensors are rigid bodies, whether they are static or dynamic.

/// Check if entities leave boundaries of physics space.
mod out_of_bounds_teleportation;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Rigid body resources.
pub mod rigid_body;
/// Link rigid body transforms.
mod rigidbody_link_transform;
/// Base rigid body spawner.
pub mod spawn;
