use bevy::{prelude::{FromWorld, World, Entity}};

use crate::space::core::pawn::functions::new_chat_message::NEARBY_SHOUT_FONT;

pub struct MOTD {
    pub message : String,
}

const COMMUNITY_HREF_COLOR : &str = "#5c4aff";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

impl FromWorld for MOTD {
    fn from_world(_world: &mut World) -> Self {
        MOTD {
            message :  "[center]".to_string() +
            "[font=" + NEARBY_SHOUT_FONT + "][color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://starwolves.io\"}]STARWOLVES.IO[/url][/color][/font]\n" +
            "Welcome to the official test server of Space Frontiers. (v" + VERSION + ")\n\n" +
            "You are about to board The Bullseye, a research & development ship currently occupied by the Red Sun Nation.\n\n" +
            "The Star Wolves gaming community is thrilled to have you here, you are invited to connect with our new gaming community through our social platforms!\n" + 
            "[font=" + NEARBY_SHOUT_FONT + "][color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://starwolves.io\"}]StarWolves.io[/url][/color][/font]\n" +
            "[/center]",
        }
    }
}

pub struct TickRate {
    pub rate : u8
}


impl FromWorld for TickRate {
    fn from_world(_world: &mut World) -> Self {
        TickRate {
           rate : 24, 
        }
    }
}

// Used for client, we can send this ID as an entityUpdate to the client which indicates it does not belong
// to a specific entity and it should be customly assigned to something such as UIs and other stuff which
// are not real server entities but just client GUI instances.
pub struct ServerId {
    pub id : Entity
}


impl FromWorld for ServerId {
    fn from_world(_world: &mut World) -> Self {
        ServerId {
           id : Entity::from_raw(0), 
        }
    }
}