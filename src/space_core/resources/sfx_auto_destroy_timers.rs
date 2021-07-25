use bevy::{core::Timer, prelude::{Entity, FromWorld, World}};

pub struct SfxAutoDestroyTimers {
    pub timers : Vec<(Entity, Timer)>
}


impl FromWorld for SfxAutoDestroyTimers {
    fn from_world(_world: &mut World) -> Self {
        SfxAutoDestroyTimers {
            timers : vec![],
        }
    }
}
