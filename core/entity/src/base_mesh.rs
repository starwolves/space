use crate::{
    entity_types::EntityType,
    spawn::{PawnId, ServerEntityClientEntity, SpawnEntity},
};
use bevy::log::warn;
use bevy::{
    prelude::{AssetServer, Commands, EventReader, Res, Visibility},
    scene::SceneBundle,
};

pub fn link_base_mesh<T: Send + Sync + 'static + EntityType + Default>(
    asset_server: Res<AssetServer>,
    mut spawner: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    id: Res<PawnId>,
    map: Res<ServerEntityClientEntity>,
) {
    for spawn in spawner.read() {
        let entity_type = T::default();

        if !spawn.entity_type.is_type(entity_type.get_identity()) {
            continue;
        }

        let mesh = asset_server.load(
            "entities/".to_string()
                + &entity_type.get_clean_identity()
                + "/client_asset.glb#Scene0",
        );

        let mut visibility = Visibility::default();

        match id.server {
            Some(o) => match map.map.get(&o) {
                Some(op) => {
                    if spawn.spawn_data.entity == *op {
                        // Own pawn entity.
                        visibility = Visibility::Hidden;
                    }
                    match spawn.spawn_data.holder_entity_option {
                        Some(s) => match map.map.get(&s) {
                            Some(ss) => {
                                if ss == op {
                                    visibility = Visibility::Hidden;
                                }
                            }
                            None => {
                                warn!("Couldnt find linked client entity.");
                            }
                        },
                        None => {}
                    }
                }
                None => {
                    warn!("Coudlnt find map map link.");
                }
            },
            None => {
                warn!("pawnid not yet set.");
            }
        }

        commands
            .entity(spawn.spawn_data.entity)
            .insert(SceneBundle {
                scene: mesh,
                transform: spawn.spawn_data.entity_transform,
                visibility,
                ..Default::default()
            });
    }
}
