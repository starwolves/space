use bevy::prelude::{
    info, warn, Commands, EventWriter, KeyCode, Query, Res, ResMut, Transform, With,
};
use entity::spawn::{EntityBuildData, PawnId, SpawnEntity};
use pawn::pawn::Pawn;
use resources::{
    hud::{EscapeMenuState, HudState},
    input::{InputBuffer, KeyBind, KeyBinds, KeyCodeEnum},
    ui::MainMenuState,
};

use crate::spawn::BallType;

pub const SHOOT_BALL_BIND: &str = "shootBall";

pub(crate) fn register_input(mut keys: ResMut<KeyBinds>) {
    keys.list.insert(
        SHOOT_BALL_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::F),
            description: "Shoot a ball.".to_string(),
            name: "Shoot ball".to_string(),
            customizable: false,
        },
    );
}

pub(crate) fn shoot_ball(
    input: Res<InputBuffer>,
    main_menu: Res<MainMenuState>,
    hud_state: Res<HudState>,
    esc_state: Res<EscapeMenuState>,
    mut commands: Commands,
    pawn_id: Res<PawnId>,
    pawn_query: Query<&Transform, With<Pawn>>,
    mut spawner: EventWriter<SpawnEntity<BallType>>,
) {
    if main_menu.enabled || hud_state.expanded || esc_state.visible {
        return;
    }
    if input.just_pressed(SHOOT_BALL_BIND) {
        let pawn_entity;
        match pawn_id.client {
            Some(e) => {
                pawn_entity = e;
            }
            None => {
                return;
            }
        }

        let pawn_transform;
        match pawn_query.get(pawn_entity) {
            Ok(t) => {
                pawn_transform = t.clone();
            }
            Err(_) => {
                warn!("Couldnt find pawn components.");
                return;
            }
        }

        let unit_rotation = pawn_transform.rotation.to_axis_angle().0;

        let mut ball_transform = pawn_transform.clone();
        ball_transform.translation += unit_rotation;

        info!("Spawning ball.");

        spawner.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity_transform: ball_transform,
                entity: commands.spawn(()).id(),
                ..Default::default()
            },
            entity_type: BallType::default(),
        });
    }
}
