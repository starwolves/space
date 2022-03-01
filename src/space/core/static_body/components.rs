use bevy_ecs::prelude::Component;
use bevy_transform::components::Transform;

#[derive(Copy, Clone, Component)]
pub struct StaticTransform {
    pub transform: Transform,
}
