use bevy::{core::Timer, prelude::{Commands,  EventWriter, Query, RemovedComponents, ResMut, info}};

use crate::space_core::{components::{boarding::Boarding, connected_player::ConnectedPlayer, on_board::OnBoard, persistent_player_data::PersistentPlayerData, setup_phase::SetupPhase, soft_player::SoftPlayer, spawning::Spawning}, events::net::{net_done_boarding::NetDoneBoarding}, resources::{asana_boarding_announcements::AsanaBoardingAnnouncements,  spawn_points::SpawnPoints}, structs::network_messages::{ReliableServerMessage,ServerConfigMessage}};

pub fn done_boarding(
    mut spawn_points : ResMut<SpawnPoints>,
    players_done_boarding: RemovedComponents<Boarding>,
    query : Query<(&SetupPhase, &ConnectedPlayer, &PersistentPlayerData)>,
    mut net_done_boarding: EventWriter<NetDoneBoarding>,
    mut commands : Commands,


    mut asana_boarding_announcements : ResMut<AsanaBoardingAnnouncements>
) {

    for entity_id in players_done_boarding.iter() {

        let (_setup_phase, connected_player, persistent_player_data) = 
        query.get(entity_id)
        .expect("done_boarding.rs could not find components for player that just got done boarding.");

        info!("{} [{}] has boarded the spaceship.",persistent_player_data.character_name, connected_player.handle);

        let assigned_spawn_transform = spawn_points.list[spawn_points.i].transform;

        commands.entity(entity_id)
        .insert_bundle((OnBoard,Spawning { transform:assigned_spawn_transform }))
        .remove_bundle::<(SetupPhase, SoftPlayer)>();

        spawn_points.i+=1;

        if spawn_points.i >= spawn_points.list.len() {
            spawn_points.i = 0;
        }

        // Queue net_code message for client so he goes back to the main scene and ditches setupUI.
        net_done_boarding.send(NetDoneBoarding {
            handle : connected_player.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(true, "main".to_string()))
        });

        asana_boarding_announcements.announcements.insert(
            ";Security Officer ".to_owned() + &persistent_player_data.character_name + " is now on board.", 
            Timer::from_seconds(2., false)
        );



    }

}
