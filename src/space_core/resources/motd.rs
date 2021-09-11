use bevy::prelude::{FromWorld, World};

use crate::space_core::functions::entity::new_chat_message::COMMUNITY_HREF_COLOR;


pub struct MOTD {
    pub message : String,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

impl FromWorld for MOTD {
    fn from_world(_world: &mut World) -> Self {
        MOTD {
            message :  "[center]".to_string() +
            "Welcome to the official test server of Space Frontiers. (v" + VERSION + ")\n\n" +
            "You are about to board The Bullseye, a research & development ship currently occupied by the Red Sun Nation.\n\n" +
            "This official server is ran by the brand new Star Wolves gaming community, to connect with the community and its amazing members visit our socials!\n\n" + 
            "StarWolves.io Community Socials:\n" +
            "[color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://starwolves.io\"}]Website & Forum[/url][/color]\n" + 
            "[color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://discord.gg/g9dtZNx8HV\"}]Discord[/url][/color]\n" + 
            "[/center]",
        }
    }
}
