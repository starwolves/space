//! Where shared code is put, useful in order to avoid cyclic dependency issues.
//! However, this ugly crate can and must be reduced a lot.

pub mod chat;
pub mod entity_updates;
pub mod health;
pub mod humanoid;
pub mod inventory;
pub mod player_controller;
pub mod rigid_body;
