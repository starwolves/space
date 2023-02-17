//! Inventory management of entities.
//! A storage component for inventory items.
//! Not all inventory holding entities are humanoids or pawns.
//! Includes actions related to the inventory system.
//! Also includes items. Item entities have special interactions with entities that hold an inventory component.
//! All inventory items can be stored inside inventories.

/// Manage inventory entity updates, such as attaching items to other items.
mod entity_update;
/// Contains inventory data.
pub mod inventory;
/// The networking module of this crate.
pub mod net;
/// The Bevy plugin of this crate.
pub mod plugin;

/// Combat resources of items.
pub mod combat;
/// Manage entity updates for inventory items.
pub mod entity_update_item;
/// Components.
pub mod item;
/// Base spawner of inventory items.
pub mod spawn_item;
