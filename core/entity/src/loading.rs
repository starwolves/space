use bevy::prelude::info;
use bevy::prelude::warn;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::ResMut;
use bevy::prelude::Transform;
use networking::client::IncomingReliableServerMessage;

use bevy::prelude::Res;

use crate::entity_types::EntityType;
use crate::entity_types::EntityTypes;
use crate::net::EntityServerMessage;
use crate::spawn::ClientEntityServerEntity;
use crate::spawn::EntityBuildData;
use crate::spawn::SpawnEntity;

/// Client loads in entities.
pub fn load_entity<T: Send + Sync + 'static + Default + EntityType>(
    mut client: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
    mut spawn_events: EventWriter<SpawnEntity<T>>,
    mut commands: Commands,
    mut map: ResMut<ClientEntityServerEntity>,
) {
    for message in client.iter() {
        match &message.message {
            EntityServerMessage::LoadEntity(load_entity) => {
                let index = types
                    .netcode_types
                    .values()
                    .position(|r| r == &load_entity.type_id)
                    .unwrap();
                let keys: Vec<&String> = types.netcode_types.keys().collect();
                let identity;
                match keys.get(index) {
                    Some(i) => identity = i.to_string(),
                    None => {
                        warn!("Coudlnt find entity type in map.");
                        continue;
                    }
                }

                let transform = Transform {
                    translation: load_entity.translation,
                    scale: load_entity.scale,
                    rotation: load_entity.rotation,
                };

                let entity_default = T::default();

                if entity_default.is_type(identity.clone()) {
                    let c_id = commands.spawn(()).id();

                    map.map.insert(load_entity.entity, c_id);
                    info!(
                        "Spawning {} sid:{:?}, cid:{:?}",
                        identity, load_entity.entity, c_id
                    );

                    spawn_events.send(SpawnEntity {
                        spawn_data: EntityBuildData {
                            entity_transform: transform,
                            correct_transform: false,
                            holder_entity_option: load_entity.holder_entity,
                            entity: c_id,
                            ..Default::default()
                        },
                        entity_type: entity_default,
                    });
                }
            }
            _ => {}
        }
    }
}
