use bevy::prelude::{App, Plugin};

use crate::set_icon::set_window_icon;

pub struct WinitWindowsPlugin;
impl Plugin for WinitWindowsPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "client") {
            app.add_startup_system(set_window_icon);
        }
    }
}
