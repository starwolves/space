use bevy::prelude::Component;
use bevy_xpbd_3d::prelude::CoefficientCombine;

pub const STANDARD_BODY_FRICTION: f32 = 0.5;

/// Component holding rigid body data.
#[derive(Component)]

pub struct RigidBodyData {
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub friction_combine_rule: CoefficientCombine,
}
