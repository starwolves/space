use bevy_ecs::entity::Entity;

#[derive(Default)]
pub struct SfxAutoDestroyTimers {
    pub timers: Vec<(Entity, u8)>,
}
