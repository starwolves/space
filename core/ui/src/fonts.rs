use std::collections::HashMap;

use bevy::{
    prelude::{AssetServer, Handle, Res, ResMut, Resource},
    text::Font,
};
use resources::modes::{is_server, AppMode};

pub const ARIZONE_FONT: &str = "fonts/ArizoneUnicaseRegular.ttf";
pub const EMPIRE_FONT: &str = "fonts/AAbsoluteEmpire.ttf";
pub const NESATHOBERYL_FONT: &str = "fonts/Nesathoberyl.ttf";
pub const SOURCECODE_REGULAR_FONT: &str = "fonts/SourceCodePro-Regular.otf";
pub const FONT_AWESOME: &str = "fonts/FontAwesome6Free-Solid-900.otf";
#[derive(Resource, Default)]
pub struct Fonts {
    i: u16,
    pub map: HashMap<u16, String>,
    pub inv_map: HashMap<String, u16>,
    pub handles: HashMap<String, Handle<Font>>,
}

impl Fonts {
    pub fn add(&mut self, path: String, asset_server: &Res<AssetServer>, app_mode: &Res<AppMode>) {
        self.map.insert(self.i, path.clone());
        self.inv_map.insert(path.clone(), self.i);
        if !(is_server() || matches!(**app_mode, AppMode::Correction)) {
            self.handles.insert(path.clone(), asset_server.load(path));
        }
        self.i += 1;
    }
}

pub fn init_fonts(
    mut fonts: ResMut<Fonts>,
    asset_server: Res<AssetServer>,
    app_mode: Res<AppMode>,
) {
    fonts.add(ARIZONE_FONT.to_string(), &asset_server, &app_mode);
    fonts.add(EMPIRE_FONT.to_string(), &asset_server, &app_mode);
    fonts.add(NESATHOBERYL_FONT.to_string(), &asset_server, &app_mode);
    fonts.add(
        SOURCECODE_REGULAR_FONT.to_string(),
        &asset_server,
        &app_mode,
    );
    fonts.add(FONT_AWESOME.to_string(), &asset_server, &app_mode);
}
