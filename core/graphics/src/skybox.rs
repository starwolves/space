use bevy::prelude::{AssetServer, Commands, Handle, Image, Res, Resource};

#[derive(Resource)]
pub struct SkyboxHandle {
    pub h: Handle<Image>,
}

pub(crate) fn preload_skybox(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(SkyboxHandle {
        h: asset_server.load("textures/skybox/starmap_2020_8k.ktx2"),
    });
}
