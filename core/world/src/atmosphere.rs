use bevy::prelude::{Commands, Vec3};
use bevy_atmosphere::prelude::{AtmosphereModel, Nishita};

pub(crate) fn add_atmosphere(mut commands: Commands) {
    commands.insert_resource(AtmosphereModel::new(Nishita {
        sun_position: Vec3::new(0., 0., -1.),
        ..Default::default()
    }));
}
