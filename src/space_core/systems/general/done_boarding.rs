use bevy::{core::Timer, prelude::{Commands, EventReader, EventWriter, ResMut, info}};

use crate::space_core::{components::{ on_board::OnBoard,  setup_phase::SetupPhase, soft_player::SoftPlayer, spawning::Spawning}, events::{general::boarding_player::BoardingPlayer, net::{net_done_boarding::NetDoneBoarding}}, resources::{asana_boarding_announcements::AsanaBoardingAnnouncements,  spawn_points::SpawnPoints}, structs::network_messages::{ReliableServerMessage,ServerConfigMessage}};

pub fn done_boarding(
    mut spawn_points : ResMut<SpawnPoints>,
    mut net_done_boarding: EventWriter<NetDoneBoarding>,
    mut boarding_player_event : EventReader<BoardingPlayer>,
    mut commands : Commands,


    mut asana_boarding_announcements : ResMut<AsanaBoardingAnnouncements>
) {

    for boarding_player in boarding_player_event.iter() {

        let player_character_name = boarding_player.player_character_name.clone();
        let player_handle = boarding_player.player_handle;
        let entity_id = boarding_player.entity;

        info!("{} [{}] has boarded the spaceship.",player_character_name, player_handle);

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
            handle : player_handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(true, "main".to_string()))
        });

        asana_boarding_announcements.announcements.insert(
            ";Security Officer ".to_owned() + &player_character_name + " is now on board.", 
            Timer::from_seconds(2., false)
        );



    }

}
