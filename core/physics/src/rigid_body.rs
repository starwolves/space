use bevy::prelude::Component;
use bevy_rapier3d::prelude::CoefficientCombineRule;

pub const STANDARD_BODY_FRICTION: f32 = 0.125;

/// Component holding rigid body data.
#[derive(Component)]

pub struct RigidBodyData {
    pub friction: f32,
    pub friction_combine_rule: CoefficientCombineRule,
}
