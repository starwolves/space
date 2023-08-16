use bevy::prelude::{
    warn, Camera3d, Commands, EventWriter, KeyCode, Query, Res, ResMut, Transform, With,
};
use cameras::LookTransform;
use entity::spawn::{EntityBuildData, SpawnEntity};
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
    camera_query: Query<&LookTransform, With<Camera3d>>,
    mut spawner: EventWriter<SpawnEntity<BallType>>,
) {
    if main_menu.enabled || hud_state.expanded || esc_state.visible {
        return;
    }
    if input.just_pressed(SHOOT_BALL_BIND) {
        let camera_transform;
        match camera_query.get_single() {
            Ok(t) => {
                camera_transform = t.clone();
            }
            Err(_) => {
                warn!("Couldnt find pawn components.");
                return;
            }
        }

        let offset = (camera_transform.target - camera_transform.eye).normalize();

        spawner.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity_transform: Transform::from_translation(camera_transform.eye + offset * 2.),
                entity: commands.spawn(()).id(),
                ..Default::default()
            },
            entity_type: BallType::default(),
        });
    }
}
