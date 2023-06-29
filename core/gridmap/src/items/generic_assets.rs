use bevy::{
    gltf::GltfMesh,
    prelude::{
        AlphaMode, AssetServer, Assets, Color, Handle, Res, ResMut, Resource, StandardMaterial,
    },
};

#[derive(Default, Resource)]
pub struct GenericMeshes {
    pub wall: Handle<GltfMesh>,
    pub floor: Handle<GltfMesh>,
    pub diagonal_template: Handle<GltfMesh>,
    pub half_diagonal_template_low: Handle<GltfMesh>,
    pub half_diagonal_template_high: Handle<GltfMesh>,
}

pub(crate) fn init_generic_meshes(
    mut res: ResMut<GenericMeshes>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mats: ResMut<GenericMaterials>,
) {
    res.wall = assets.load("models/wall_template/wall.glb#Mesh0");
    res.floor = assets.load("models/floor_template/floor.glb#Mesh0");
    res.diagonal_template = assets.load("models/diagonal_template/diagonal_template.glb#Mesh0");
    res.half_diagonal_template_low =
        assets.load("models/half_diagonal_template/half_diagonal_template_low.glb#Mesh0");
    res.half_diagonal_template_high =
        assets.load("models/half_diagonal_template/half_diagonal_template_high.glb#Mesh0");

    let mat = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 1., 0., 0.5),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    mats.glass = mat;
}

#[derive(Default, Resource)]
pub struct GenericMaterials {
    pub gray_metallic: Handle<StandardMaterial>,
    pub glass: Handle<StandardMaterial>,
}

pub(crate) fn init_default_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mat: ResMut<GenericMaterials>,
) {
    let m = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.9,
        metallic: 0.7,
        ..Default::default()
    });
    mat.gray_metallic = m.clone();
}
