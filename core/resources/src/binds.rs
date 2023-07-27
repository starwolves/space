use std::collections::HashMap;

use bevy::prelude::{KeyCode, Resource};

#[derive(Default, Resource)]
pub struct KeyBinds {
    pub list: HashMap<String, KeyBind>,
}
#[derive(Clone)]
pub struct KeyBind {
    pub key_code: KeyCode,
    pub description: String,
    pub name: String,
    pub customizable: bool,
}

impl KeyBinds {
    pub fn bind(&self, id: &str) -> KeyCode {
        self.list.get(id).unwrap().key_code
    }
}
