//! For items and gridmap cells that can be examined.
//! Examining is modular and allows systems to tap in to expand the examine messages. Therefore this logic relies on correct usage of system labels and ordering.
//! The examine functionality is integrated as an action and gets executed through the actions API.

/// Hook examine actions.
mod actions;
/// Perform examine actions.
pub mod examine;
/// The Bevy plugin of this crate.
pub mod plugin;
