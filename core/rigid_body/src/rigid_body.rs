use bevy::prelude::Component;
use bevy_rapier3d::prelude::CoefficientCombineRule;

#[cfg(feature = "server")]
pub const STANDARD_BODY_FRICTION: f32 = 0.125;

/// Component holding rigid body data.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct RigidBodyData {
    pub friction: f32,
    pub friction_combine_rule: CoefficientCombineRule,
}
