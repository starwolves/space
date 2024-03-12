use bevy::ecs::event::EventReader;
use bevy::log::warn;
use bevy::prelude::{Commands, EventWriter, KeyCode, Query, Res, ResMut, Transform, With};
use cameras::LookTransform;
use entity::spawn::{EntityBuildData, SpawnEntity};
use networking::client::OutgoingReliableClientMessage;
use networking::server::{HandleToEntity, IncomingReliableClientMessage};
use pawn::pawn::Pawn;
use resources::{
    hud::{EscapeMenuState, HudState},
    input::{InputBuffer, KeyBind, KeyBinds, KeyCodeEnum},
    ui::MainMenuState,
};

use crate::net::BallClientMessage;
use crate::spawn::BallType;

pub const SHOOT_BALL_BIND: &str = "shootBall";

pub(crate) fn register_input(mut keys: ResMut<KeyBinds>) {
    keys.list.insert(
        SHOOT_BALL_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::KeyF),
            description: "Shoot a ball.".to_string(),
            name: "Shoot ball".to_string(),
            customizable: false,
        },
    );
}

pub(crate) fn shoot_ball_client(
    input: Res<InputBuffer>,
    main_menu: Res<MainMenuState>,
    hud_state: Res<HudState>,
    esc_state: Res<EscapeMenuState>,

    mut shoot: EventWriter<OutgoingReliableClientMessage<BallClientMessage>>,
) {
    if main_menu.enabled || hud_state.expanded || esc_state.visible {
        return;
    }
    if input.just_pressed(SHOOT_BALL_BIND) {
        shoot.send(OutgoingReliableClientMessage {
            message: BallClientMessage::Shoot,
        });
    }
}

pub(crate) fn shoot_ball_server(
    mut events: EventReader<IncomingReliableClientMessage<BallClientMessage>>,
    mut commands: Commands,
    pawn_query: Query<(&LookTransform, &Transform), With<Pawn>>,
    mut spawner: EventWriter<SpawnEntity<BallType>>,
    handles: Res<HandleToEntity>,
) {
    for message in events.read() {
        match message.message {
            BallClientMessage::Shoot => {
                let entity;
                match handles.map.get(&message.handle) {
                    Some(ent) => {
                        entity = ent;
                    }
                    None => {
                        warn!("Couldnt find handle entity.");
                        continue;
                    }
                }
                let (camera_transform, entity_transform);

                match pawn_query.get(*entity) {
                    Ok(t) => {
                        camera_transform = t.0.clone();
                        entity_transform = t.1.clone();
                    }
                    Err(_) => {
                        warn!("Couldnt find pawn components.");
                        return;
                    }
                }

                let offset = (camera_transform.target - camera_transform.eye).normalize();
                let new = commands.spawn(()).id();
                spawner.send(SpawnEntity {
                    spawn_data: EntityBuildData {
                        entity_transform: Transform::from_translation(
                            entity_transform.translation + camera_transform.eye + offset * 2.,
                        ),
                        entity: Some(new),
                        ..Default::default()
                    },
                    entity_type: BallType::default(),
                });
            }
        }
    }
}
