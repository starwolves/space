use std::f32::consts::PI;

use bevy::{
    gltf::GltfMesh,
    prelude::{Handle, Quat, Res, ResMut, Transform, Vec3},
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;

pub(crate) fn init_generic_diagonal_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    app_mode: Res<AppMode>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.diagonal_template.clone_weak());
    } else {
        mesh_option = None;
    }

    let mut rot = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.5 * PI);
    rot *= Quat::from_axis_angle(Vec3::new(0., 1., 0.), 0.75 * PI);

    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_diagonal_floor".to_string()),
        name: RichName {
            name: "diagonal aluminum floor".to_string(),
            n: true,
            the: false,
        },
        description: "A generic diagonal floor tile.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Center,
        x_rotations: vec![0, 16, 3, 19],
        vertical_rotation: false,
        collider: Collider::cuboid(1.415, 1., 0.285),
        collider_position: Transform {
            translation: Vec3::new(-0.1, 0., 0.),
            rotation: rot,
            ..Default::default()
        },
        ..Default::default()
    });
}
