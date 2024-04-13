use bevy::pbr::PointLight;

pub fn default_point_light() -> PointLight {
    PointLight {
        shadows_enabled: true,
        intensity: 700_000_000.,
        ..Default::default()
    }
}
