use bevy::{
    prelude::{warn, AssetServer, Commands, EventReader, Res, Visibility},
    scene::SceneBundle,
};

use crate::{
    entity_types::EntityType,
    spawn::{ClientEntityServerEntity, PawnEntityId, SpawnEntity},
};

pub fn link_base_mesh<T: Send + Sync + 'static + EntityType + Default>(
    asset_server: Res<AssetServer>,
    mut spawner: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    id: Res<PawnEntityId>,
    map: Res<ClientEntityServerEntity>,
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

        let mut visibility = Visibility::default();

        match id.option {
            Some(o) => match map.map.get(&o) {
                Some(op) => {
                    if spawn.spawn_data.entity == *op {
                        // Own pawn entity.
                        //visibility.is_visible = false;
                    }
                    match spawn.spawn_data.holder_entity_option {
                        Some(s) => match map.map.get(&s) {
                            Some(ss) => {
                                if ss == op {
                                    visibility.is_visible = false;
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
