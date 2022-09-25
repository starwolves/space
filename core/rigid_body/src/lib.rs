//! Bevy rapier rigid body integration.

/// Check if entities leave boundaries of physics space.
mod out_of_bounds_check;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Rigid body resources.
pub mod rigid_body;
/// Link rigid body transforms.
mod rigidbody_link_transform;
/// Base rigid body spawner.
pub mod spawn;
