use std::collections::HashMap;

use bevy::prelude::{ResMut, Resource};

pub const ARIZONE_FONT: &str = "fonts/ArizoneUnicaseRegular.ttf";
pub const EMPIRE_FONT: &str = "fonts/AAbsoluteEmpire.ttf";
pub const NESATHOBERYL_FONT: &str = "fonts/Nesathoberyl.ttf";

#[derive(Resource, Default)]
pub struct Fonts {
    i: u16,
    pub map: HashMap<u16, String>,
    pub inv_map: HashMap<String, u16>,
}

impl Fonts {
    pub fn add(&mut self, path: String) {
        self.map.insert(self.i, path.clone());
        self.inv_map.insert(path, self.i);
    }
}

pub(crate) fn init_fonts(mut fonts: ResMut<Fonts>) {
    fonts.add(ARIZONE_FONT.to_string());
    fonts.add(EMPIRE_FONT.to_string());
    fonts.add(NESATHOBERYL_FONT.to_string());
}
