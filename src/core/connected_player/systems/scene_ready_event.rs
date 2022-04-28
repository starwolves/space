use bevy_app::EventReader;
use bevy_ecs::{
    prelude::Without,
    system::{Commands, Query, Res},
};

use crate::core::connected_player::{
    components::{Boarding, SetupPhase, SoftPlayer},
    events::InputSceneReady,
    resources::HandleToEntity,
};

pub fn scene_ready_event(
    mut event: EventReader<InputSceneReady>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&SoftPlayer, Without<Boarding>>,
    mut commands: Commands,
) {
    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("scene_ready_event.rs could not find components for player that just got done boarding.");

        //Safety check.
        match criteria_query.get(*player_entity) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        if new_event.scene_type == "setupUI" {
            commands.entity(*player_entity).insert(SetupPhase);
        }
    }
}
