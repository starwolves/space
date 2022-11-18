//! Actions enable pawns to interact with the world and other entities.
//! There are two variants of action requests, one is to list all available actions for an entity and the other is to actually execute an action.
//! Both forms involve a modular process of obtaining one or more available action for the entity that is requesting it, running a prerequisite check and then finalizing the obtained results by prerequisite checkers. This all happens in one single frame. Therefore interacting with actions requires correct systems ordering and labelling.

/// The core action module.
pub mod core;
/// The networking module of this crate.
pub mod networking;
/// The Bevy plugin.
pub mod plugin;
