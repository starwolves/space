use std::collections::HashMap;

use bevy::{
    ecs::{schedule::SystemSet, system::Local},
    prelude::{Input, KeyCode, MouseButton, Res, ResMut, Resource},
};

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InputSet {
    Prepare,
    Cache,
    ApplyLiveCache,
}
pub const MOVE_FORWARD_BIND: &str = "moveForward";
pub const MOVE_BACKWARD_BIND: &str = "moveBackward";
pub const MOVE_LEFT_BIND: &str = "moveLeft";
pub const MOVE_RIGHT_BIND: &str = "moveRight";
pub const JUMP_BIND: &str = "jump";
pub const HOLD_SPRINT_BIND: &str = "holdSprint";
#[derive(Clone, Debug)]
pub struct InputPart {
    pub bind: KeyBind,
    pub pressed: bool,
    pub id: String,
}
#[derive(Resource, Default, Clone)]
pub struct InputBuffer {
    pub buffer: HashMap<String, InputPart>,
    pub pressed: HashMap<String, bool>,
}

impl InputBuffer {
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
    pub fn add_input(&mut self, p: InputPart) {
        self.buffer.insert(p.id.clone(), p.clone());
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
        match self.buffer.get(id) {
            Some(t) => t.pressed,
            None => false,
        }
    }
    pub fn just_released(&self, id: &str) -> bool {
        match self.buffer.get(id) {
            Some(t) => !t.pressed,
            None => false,
        }
    }
}

pub(crate) fn clear_buffer(mut buffer: ResMut<InputBuffer>) {
    buffer.clear();
}
#[derive(Resource, Default)]
pub(crate) struct LastBuffer(InputBuffer);
pub(crate) fn sanitize_input(mut local: Local<LastBuffer>, mut buffer: ResMut<InputBuffer>) {
    for (id, pressed) in buffer.clone().pressed.iter() {
        match local.0.pressed.get(id) {
            Some(p) => {
                if pressed == p {
                    buffer.buffer.remove(id);
                }
            }
            None => {}
        }
    }

    local.0 = buffer.clone();
}

pub(crate) fn buffer_input(
    keys: Res<KeyBinds>,
    mut buffer: ResMut<InputBuffer>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    for (id, bind) in keys.list.iter() {
        match bind.key_code {
            KeyCodeEnum::Keyboard(k) => {
                if keyboard.just_pressed(k) {
                    buffer.add_input(InputPart {
                        bind: bind.clone(),
                        pressed: keyboard.pressed(k),
                        id: id.clone(),
                    });
                } else if keyboard.just_released(k) {
                    buffer.add_input(InputPart {
                        bind: bind.clone(),
                        pressed: keyboard.pressed(k),
                        id: id.clone(),
                    });
                }
            }
            KeyCodeEnum::Mouse(m) => {
                if mouse.just_pressed(m) {
                    buffer.add_input(InputPart {
                        bind: bind.clone(),
                        pressed: true,
                        id: id.clone(),
                    });
                } else if mouse.just_released(m) {
                    buffer.add_input(InputPart {
                        bind: bind.clone(),
                        pressed: false,
                        id: id.clone(),
                    });
                }
            }
        }
    }
}

#[derive(Default, Resource)]
pub struct KeyBinds {
    pub list: HashMap<String, KeyBind>,
}
#[derive(Clone, Debug)]
pub struct KeyBind {
    pub key_code: KeyCodeEnum,
    pub description: String,
    pub name: String,
    pub customizable: bool,
}
#[derive(Clone, Debug)]
pub enum KeyCodeEnum {
    Keyboard(KeyCode),
    Mouse(MouseButton),
}

impl KeyBinds {
    pub fn keyboard_bind(&self, id: &str) -> KeyCode {
        match self.list.get(id).unwrap().key_code {
            KeyCodeEnum::Keyboard(t) => t,
            KeyCodeEnum::Mouse(_) => {
                panic!("Not a keyboard bind.");
            }
        }
    }
    pub fn mouse_bind(&self, id: &str) -> MouseButton {
        match self.list.get(id).unwrap().key_code {
            KeyCodeEnum::Keyboard(_) => {
                panic!("Not a mouse bind.");
            }
            KeyCodeEnum::Mouse(t) => t,
        }
    }
}
#[derive(Resource, Default)]
pub struct IsFixedUpdateTick(pub bool);
pub(crate) fn set_fixed_update_tick(mut tick: ResMut<IsFixedUpdateTick>) {
    tick.0 = true;
}
pub(crate) fn clear_fixed_update_tick(mut tick: ResMut<IsFixedUpdateTick>) {
    tick.0 = false;
}
