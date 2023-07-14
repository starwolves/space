/*use std::f32::consts::PI;

use bevy::prelude::{
    AmbientLight, Commands, DirectionalLight, DirectionalLightBundle, Quat, Transform, Vec3,
};
use bevy_atmosphere::prelude::{AtmosphereModel, Nishita};

pub(crate) fn add_atmosphere(mut commands: Commands) {
    commands.insert_resource(AtmosphereModel::new(Nishita {
        sun_position: Vec3::new(0., 0., -1.),
        ..Default::default()
    }));
    commands.insert_resource(AmbientLight {
        brightness: 0.,
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_y(-PI * 1.).mul_quat(Quat::from_rotation_x(-PI * 0.1)),
            ..Default::default()
        },
        ..Default::default()
    });
}
*/
