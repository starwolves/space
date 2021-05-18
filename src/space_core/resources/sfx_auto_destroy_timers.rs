use std::collections::HashMap;

use bevy::{core::Timer, prelude::Entity};

pub struct SfxAutoDestroyTimers {
    pub timers : HashMap<Entity, Timer>
}
