use bevy_internal::prelude::{Entity, FromWorld, World};

pub struct SfxAutoDestroyTimers {
    pub timers: Vec<(Entity, u8)>,
}

impl FromWorld for SfxAutoDestroyTimers {
    fn from_world(_world: &mut World) -> Self {
        SfxAutoDestroyTimers { timers: vec![] }
    }
}
