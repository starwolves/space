use bevy::{
    app::AppExit,
    prelude::{EventWriter, Input, KeyCode, Res},
};

pub(crate) fn quit_application(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
