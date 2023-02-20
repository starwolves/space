use bevy::{
    prelude::{AssetServer, Commands, EventReader, Res, Transform},
    scene::SceneBundle,
};

use crate::{entity_types::EntityType, spawn::SpawnEntity};

pub fn link_base_mesh<T: Send + Sync + 'static + EntityType + Default>(
    asset_server: Res<AssetServer>,
    mut spawner: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn in spawner.iter() {
        let entity_type = T::default();

        if !spawn.entity_type.is_type(entity_type.get_identity()) {
            continue;
        }

        let mesh = asset_server.load(
            "entities/".to_string()
                + &entity_type.get_clean_identity()
                + "/client_asset.glb#Scene0",
        );

        let mut transform = Transform::IDENTITY;
        transform.translation.y = 2.;
        commands
            .entity(spawn.spawn_data.entity)
            .insert(SceneBundle {
                scene: mesh,
                transform: transform,
                ..Default::default()
            });
    }
}
