use bevy::prelude::{Commands, EventReader, Res};

use crate::space_core::{components::setup_phase::SetupPhase, events::general::scene_ready::InputSceneReady, resources::handle_to_entity::HandleToEntity};

pub fn scene_ready_event(
    mut event : EventReader<InputSceneReady>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands : Commands
) {

    for new_event in event.iter() {

        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("scene_ready_event.rs could not find components for player that just got done boarding.");


        if new_event.scene_type == "setupUI" {

            commands.entity(*player_entity).insert(SetupPhase);
    
        }


    }


}
