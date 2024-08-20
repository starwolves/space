use bevy::{
    asset::Handle,
    color::Color,
    ecs::system::Resource,
    math::Vec3,
    prelude::{AlphaMode, AssetServer, Assets, Res, ResMut, StandardMaterial, Transform},
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::{
    light::default_point_light,
    modes::{is_server, AppMode},
};

use crate::{
    grid::{CellType, CellTypeName, TileLight, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;
#[derive(Resource, Default)]
pub struct LightMaterials {
    pub ceiling: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
}

pub(crate) fn init_ceiling_light_bar_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<LightMaterials>,
) {
    let albedo_texture_handle =
        asset_server.load("gridmap/ceiling_light_bar/ceiling_light_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/ceiling_light_bar/ceiling_light_metal_rough.png");
    let emissive = asset_server.load("gridmap/ceiling_light_bar/ceiling_light_emissive.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        alpha_mode: AlphaMode::Blend,
        thickness: 0.2,
        ior: 1.52,
        diffuse_transmission: 1.,
        emissive_texture: Some(emissive),
        emissive: Color::srgb(100000., 100000., 100000.).into(),
        ..Default::default()
    });
    res.ceiling = material_handle;
}

pub(crate) fn init_ceiling_light_bar(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<LightMaterials>,
    app_mode: Res<AppMode>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;
    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.ceiling_light.clone_weak());

        material_option = Some(mat.ceiling.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("ceiling_light_bar".to_string()),
        name: RichName {
            name: "ceiling light bar".to_string(),
            n: true,
            the: false,
        },
        description: "A ceiling light bar.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Center,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        x_rotations: vec![0, 16, 10, 22],
        is_light: Some(TileLight {
            light: default_point_light(),
            local_offset: Vec3::new(0., -1.25, -0.),
        }),
        ..Default::default()
    });
}
