use std::f32::consts::PI;

use bevy::{
    gltf::GltfMesh,
    prelude::{
        AlphaMode, AssetServer, Assets, Handle, Quat, Res, ResMut, Resource, StandardMaterial,
        Transform, Vec3,
    },
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;

#[derive(Default, Resource)]
pub struct HalfCeilingMaterial {
    pub material_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_half_ceiling_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<HalfCeilingMaterial>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/half_ceiling/half_ceiling_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/half_ceiling/half_ceiling_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        alpha_mode: AlphaMode::Mask(0.5),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        thickness: 0.2,
        ior: 1.52,
        ..Default::default()
    });
    res.material_handle = material_handle;
}

pub(crate) fn init_half_ceiling(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<HalfCeilingMaterial>,
    app_mode: Res<AppMode>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.half_ceiling.clone_weak());
        material_option = Some(mat.material_handle.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    let mut rot = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.5 * PI);
    rot *= Quat::from_axis_angle(Vec3::new(0., 1., 0.), -0.15 * PI);

    init.properties.push(TileProperties {
        name_id: CellTypeName("half_ceiling".to_string()),
        name: RichName {
            name: "half ceiling".to_string(),
            n: true,
            the: false,
        },
        description: "A half ceiling.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Center,
        vertical_rotation: false,
        x_rotations: vec![0, 16, 10, 22],
        material_option,
        collider: Collider::cuboid(1.117, 1., 0.265),
        collider_position: Transform {
            rotation: rot,
            translation: Vec3::new(-0.05, -0.272, 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
