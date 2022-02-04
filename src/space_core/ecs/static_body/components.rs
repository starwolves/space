use bevy::prelude::{Transform, Component};

#[derive(Copy, Clone, Component)]
pub struct StaticTransform {
    pub transform : Transform
}
