/*use std::path::Path;

use bevy::{
    prelude::{NonSend, Query, With},
    window::{PrimaryWindow, Window},
    winit::WinitWindows,
};
use winit::window::Icon;

pub(crate) fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary = primary_query.get_single().unwrap();

    let path = Path::new("././data/project/sflogo.png");

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}*/
