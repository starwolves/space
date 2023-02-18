//! Inventory management of entities.
//! A storage component for inventory items.
//! Not all inventory holding entities are humanoids or pawns.
//! Includes actions related to the inventory system.
//! Also includes items. Item entities have special interactions with entities that hold an inventory component.
//! All inventory items can be stored inside inventories.

/// The networking module of this crate.
pub mod net;
/// The Bevy plugin of this crate.
pub mod plugin;

pub mod client;
/// Components.
pub mod item;
pub mod server;
/// Base spawner of inventory items.
pub mod spawn_item;
