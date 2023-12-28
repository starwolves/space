use crate::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use bevy::{
    app::prelude::*,
    ecs::{bundle::Bundle, prelude::*},
    input::mouse::MouseMotion,
    math::prelude::*,
    time::Time,
    transform::components::Transform,
};
use resources::hud::HudState;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct FpsCameraPlugin {
    pub override_input_system: bool,
}

impl FpsCameraPlugin {
    pub fn new(override_input_system: bool) -> Self {
        Self {
            override_input_system,
        }
    }
}
impl Plugin for FpsCameraPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .add_systems(PreUpdate, on_controller_enabled_changed)
            .add_systems(Update, control_system)
            .add_event::<ControlEvent>()
            .init_resource::<ActiveCamera>();
        if !self.override_input_system {
            app.add_systems(Update, default_input_map.before(control_system));
        }
        // app.add_system(print_camera_position);
    }
}

#[derive(Bundle)]
pub struct FpsCameraBundle {
    controller: FpsCameraController,
    look_transform: LookTransformBundle,
    transform: Transform,
}

impl FpsCameraBundle {
    pub fn new(controller: FpsCameraController, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        // Make sure the transform is consistent with the controller to start.
        let transform = Transform::from_translation(eye).looking_at(target, up);

        Self {
            controller,
            look_transform: LookTransformBundle {
                transform: LookTransform::new(eye, target, up),
                smoother: Smoother::new(controller.smoothing_weight),
            },
            transform,
        }
    }
}

/// Your typical first-person camera controller.
#[derive(Clone, Component, Copy, Debug, Deserialize, Serialize)]
pub struct FpsCameraController {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub translate_sensitivity: f32,
    pub smoothing_weight: f32,
}

impl Default for FpsCameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(0.5),
            translate_sensitivity: 6.0,
            smoothing_weight: 0.4,
        }
    }
}

#[derive(Event)]
pub enum ControlEvent {
    Rotate(Vec2),
    TranslateEye(Vec3),
}

define_on_controller_enabled_changed!(FpsCameraController);

pub fn default_input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    controllers: Query<&FpsCameraController>,
    hud_state: Res<HudState>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    if hud_state.expanded {
        return;
    }
    let FpsCameraController {
        mouse_rotate_sensitivity,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    let mut read = false;
    for event in mouse_motion_events.read() {
        if read {
            continue;
        }
        cursor_delta = event.delta;
        read = true;
    }

    events.send(ControlEvent::Rotate(
        mouse_rotate_sensitivity * cursor_delta,
    ));

    /*for (key, dir) in [
        (binds.keyboard_bind(MOVE_FORWARD_BIND), Vec3::Z),
        (binds.keyboard_bind(MOVE_LEFT_BIND), Vec3::X),
        (binds.keyboard_bind(MOVE_BACKWARD_BIND), -Vec3::Z),
        (binds.keyboard_bind(MOVE_RIGHT_BIND), -Vec3::X),
        (binds.keyboard_bind(HOLD_SPRINT_BIND), -Vec3::Y),
        (binds.keyboard_bind(JUMP_BIND), Vec3::Y),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            events.send(ControlEvent::TranslateEye(translate_sensitivity * dir));
        }
    }*/
}

#[derive(Resource, Default)]
pub struct ActiveCamera {
    pub option: Option<Entity>,
}

pub fn control_system(
    mut events: EventReader<ControlEvent>,
    mut cameras: Query<(&FpsCameraController, &mut LookTransform)>,
    time: Res<Time>,
) {
    // Can only control one camera at a time.
    let mut transform = if let Some((_, transform)) = cameras.iter_mut().find(|c| c.0.enabled) {
        transform
    } else {
        return;
    };

    let look_vector;

    match transform.look_direction() {
        Some(v) => {
            look_vector = v;
        }
        None => {
            return;
        }
    }
    let mut look_angles = LookAngles::from_vector(look_vector);

    let yaw_rot = Quat::from_axis_angle(Vec3::Y, look_angles.get_yaw());
    let rot_x = yaw_rot * Vec3::X;
    let rot_y = yaw_rot * Vec3::Y;
    let rot_z = yaw_rot * Vec3::Z;

    let dt = time.delta_seconds();
    for event in events.read() {
        match event {
            ControlEvent::Rotate(delta) => {
                // Rotates with pitch and yaw.
                look_angles.add_yaw(dt * -delta.x);
                look_angles.add_pitch(dt * -delta.y);
            }
            ControlEvent::TranslateEye(delta) => {
                // Translates up/down (Y) left/right (X) and forward/back (Z).
                transform.eye += dt * delta.x * rot_x + dt * delta.y * rot_y + dt * delta.z * rot_z;
            }
        }
    }

    look_angles.assert_not_looking_up();

    transform.target = transform.eye + transform.radius() * look_angles.unit_vector();
}
