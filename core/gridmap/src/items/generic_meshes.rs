use bevy::{
    gltf::GltfMesh,
    prelude::{AssetServer, Commands, Handle, Res, Resource},
};

#[derive(Resource)]
pub struct GenericMeshes {
    pub wall: Handle<GltfMesh>,
    pub floor: Handle<GltfMesh>,
}

pub(crate) fn init_generic_meshes(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GenericMeshes {
        wall: assets.load("models/wall/wall.glb#Mesh0"),
        floor: assets.load("models/floor/floor.glb#Mesh0"),
    });
}
