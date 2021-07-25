use bevy::{core::Timer, prelude::Entity};

pub struct SfxAutoDestroyTimers {
    pub timers : Vec<(Entity, Timer)>
}
