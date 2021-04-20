use bevy::prelude::{Commands, Query, RemovedComponents, ResMut, info};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{components::{boarding::Boarding, connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData, setup_phase::SetupPhase}, resources::used_names::UsedNames};

pub fn done_boarding(
    mut _net: ResMut<NetworkResource>,
    mut used_names : ResMut<UsedNames>,
    players_done_boarding: RemovedComponents<Boarding>,
    query : Query<(&SetupPhase, &ConnectedPlayer, &PersistentPlayerData)>,
    mut commands : Commands
) {

    for entity_id in players_done_boarding.iter() {

        let (_setup_phase, connected_player, persistent_player_data) = 
        query.get(entity_id)
        .expect("done_boarding.rs could not find components for player that just got done boarding.");

        commands.entity(entity_id).remove::<SetupPhase>();

        used_names.names.push(persistent_player_data.character_name.clone());

        // We have the player's name, now fully spawn in the player and remove from softConnected

        info!("{} [{}] has boarded the spaceship.",persistent_player_data.character_name, connected_player.handle);

        

    }

}
