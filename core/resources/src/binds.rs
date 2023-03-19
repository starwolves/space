use std::collections::HashMap;

use bevy::prelude::{KeyCode, Resource};

#[derive(Default, Resource)]
pub struct KeyBinds {
    pub list: HashMap<String, KeyBind>,
}

pub struct KeyBind {
    pub key_code: KeyCode,
    pub description: String,
    pub name: String,
}

impl KeyBinds {
    pub fn bind(&self, id: String) -> KeyCode {
        self.list.get(&id).unwrap().key_code
    }
}
