use std::f32::consts::PI;

use bevy::{
    gltf::GltfMesh,
    prelude::{Handle, Quat, Res, ResMut, Transform, Vec3},
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;

pub(crate) fn init_generic_diagonal_ceiling(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    if !is_server() {
        mesh_option = Some(meshes.diagonal_template.clone());
    } else {
        mesh_option = None;
    }
    let mut rot = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.5 * PI);
    rot *= Quat::from_axis_angle(Vec3::new(0., 0., 1.), PI);

    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_diagonal_ceiling".to_string()),
        name: RichName {
            name: "diagonal aluminum ceiling".to_string(),
            n: true,
            the: false,
        },
        description: "A generic ceiling tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Center,
        x_rotations: vec![0, 16, 3, 19],
        vertical_rotation: false,
        collider: Collider::convex_hull(vec![
            Vec3::new(0.3, 0.5, -0.6).into(),
            Vec3::new(0.5, 0.5, -0.4).into(),
            Vec3::new(0.5, -0.5, -0.4).into(),
            Vec3::new(0.3, -0.5, -0.6).into(),
            Vec3::new(-0.5, -0.5, 0.6).into(),
            Vec3::new(-0.7, -0.5, 0.4).into(),
            Vec3::new(-0.7, 0.5, 0.4).into(),
            Vec3::new(-0.5, 0.5, 0.6).into(),
        ])
        .unwrap(),
        collider_position: Transform {
            translation: Vec3::new(-0.2, 0., 0.),
            rotation: rot,
            ..Default::default()
        },
        ..Default::default()
    });
}
