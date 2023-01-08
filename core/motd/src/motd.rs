use bevy::prelude::Resource;
use text_api::core::{COMMUNITY_HREF_COLOR, NEARBY_SHOUT_FONT};

impl MOTD {
    pub fn new_default(version: String) -> Self {
        Self {
            message :  "[center]".to_string() +
            "[font=" + NEARBY_SHOUT_FONT + "][color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://github.com/starwolves/space\"}]Space Frontiers[/url][/color][/font]\n" +
            "Welcome to the official test server of Space Frontiers (v" + &version + ").\n\n" +
            "You are about to board The Bullseye, a research & development ship.\n\n" +
            "The Space Frontiers community is thrilled to have you here, you are invited to connect with our new gaming community through our social platforms!\n" + 
            "[font=" + NEARBY_SHOUT_FONT + "][color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://github.com/starwolves/space\"}]Github[/url][/color][/font]\n" +
            "[/center]",
        }
    }
    pub fn new_motd(motd: String) -> Self {
        Self { message: motd }
    }
}

/// Resource message of the day visible to players upon connecting.

#[derive(Resource)]
pub struct MOTD {
    pub message: String,
}
