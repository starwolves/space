use std::collections::HashMap;

use bevy::prelude::{Input, KeyCode, Res, ResMut, Resource};

use crate::binds::{KeyBind, KeyBinds};

pub const MOVE_FORWARD_BIND: &str = "moveForward";
pub const MOVE_BACKWARD_BIND: &str = "moveBackward";
pub const MOVE_LEFT_BIND: &str = "moveLeft";
pub const MOVE_RIGHT_BIND: &str = "moveRight";
pub const JUMP_BIND: &str = "jump";
pub const HOLD_SPRINT_BIND: &str = "holdSprint";
#[derive(Clone)]
pub struct InputPart {
    pub bind: KeyBind,
    pub pressed: bool,
    pub id: String,
}
#[derive(Resource, Default)]
pub struct InputBuffer {
    pub buffer: Vec<InputPart>,
    pub pressed: HashMap<String, bool>,
}

impl InputBuffer {
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
    pub fn add_input(&mut self, p: InputPart) {
        self.buffer.push(p.clone());
        self.pressed.insert(p.id, p.pressed);
    }
    pub fn pressed(&self, id: &str) -> bool {
        match self.pressed.get(id) {
            Some(f) => *f,
            None => false,
        }
    }
    pub fn released(&self, id: &str) -> bool {
        match self.pressed.get(id) {
            Some(f) => !*f,
            None => true,
        }
    }
    pub fn just_pressed(&self, id: &str) -> bool {
        for p in self.buffer.iter() {
            if p.pressed {
                if id == p.id {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn just_released(&self, id: &str) -> bool {
        for p in self.buffer.iter() {
            if !p.pressed {
                if id == p.id {
                    return true;
                }
            }
        }
        return false;
    }
}

pub(crate) fn clear_buffer(mut buffer: ResMut<InputBuffer>) {
    buffer.clear();
}

pub(crate) fn buffer_input(
    keys: Res<KeyBinds>,
    mut buffer: ResMut<InputBuffer>,
    keys2: Res<Input<KeyCode>>,
) {
    for (id, bind) in keys.list.iter() {
        if keys2.just_pressed(bind.key_code) {
            buffer.add_input(InputPart {
                bind: bind.clone(),
                pressed: true,
                id: id.clone(),
            });
        } else if keys2.just_released(bind.key_code) {
            buffer.add_input(InputPart {
                bind: bind.clone(),
                pressed: false,
                id: id.clone(),
            });
        }
    }
}
