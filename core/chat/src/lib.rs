//! Chat managing proximity and radio communication.
//! Offers a lot of advanced chat features to allow for great chat customization.
//! Players can talk and listen through radios.
//! Chat has a lot of attention to minor details, like such as including the distance and the types of entities that speak as factors to stylize and edit the chat messages.

/// Manage chat, radio chat and global chat.
pub mod chat;
/// The serialized messages that get sent over the net.
pub mod net;
/// The networking module of this crate.
pub mod networking;
/// The Bevy plugin of this crate.
pub mod plugin;
