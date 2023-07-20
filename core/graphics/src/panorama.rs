use bevy::{
    gltf::GltfMesh,
    prelude::{
        warn, AssetServer, Assets, Color, Commands, Handle, Local, PbrBundle, Res, ResMut,
        Resource, StandardMaterial, Transform,
    },
};
#[derive(Resource)]
pub struct PanoramaMesh {
    pub mesh: Handle<GltfMesh>,
}

pub(crate) fn init_panorama_sphere(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(PanoramaMesh {
        mesh: asset_server.load("models/panorama_spherical/sphere.glb#Mesh0".to_string()),
    });
}

pub(crate) fn init_milky_way(
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    mut commands: Commands,
    mut loaded: Local<bool>,
    mesh: Res<PanoramaMesh>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if !*loaded {
        *loaded = true;
        let mut transform = Transform::default();
        transform.scale *= 5000.;
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(
                asset_server.load("models/panorama_spherical/milky_way_eso0932a.jpg"),
            ),
            unlit: true,
            ..Default::default()
        });
        match assets_gltfmesh.get(&mesh.mesh) {
            Some(gltf) => {
                commands.spawn(PbrBundle {
                    mesh: gltf.primitives[0].mesh.clone(),
                    transform,
                    material,
                    ..Default::default()
                });
            }
            None => {
                warn!("Couldnt find gltf mesh.");
            }
        }
    }
}
