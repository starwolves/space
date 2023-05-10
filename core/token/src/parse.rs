use std::{env, fs, path::Path};

use bevy::prelude::{warn, Commands, Resource};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct Token {
    pub token: String,
    pub name: String,
}

pub(crate) fn init_token(mut commands: Commands) {
    let args: Vec<String> = env::args().collect();

    let mut auth_i = None;
    let mut name_i = None;

    let mut i = 0;
    for arg in args.iter() {
        if arg == "auth" {
            auth_i = Some(i);
        } else if arg == "name" {
            name_i = Some(i);
        }
        i += 1;
    }

    let token;

    if auth_i.is_none() || name_i.is_none() {
        let token_dir;

        match BaseDirs::new() {
            Some(base_dir) => {
                let mut file = base_dir.data_dir().join("io.starwolves").join("token.json");
                if !file.exists() {
                    file = Path::new("token.json").to_path_buf();
                    if !file.exists() {
                        warn!("Please log in with the launcher obtained at https://store.starwolves.io .");
                        return;
                    }
                }
                token_dir = file;
            }
            None => {
                warn!("Couldnt get basedirs.");
                return;
            }
        }

        match fs::read_to_string(token_dir) {
            Ok(tj) => match serde_json::from_str::<Token>(&tj) {
                Ok(t) => {
                    token = t.clone();
                }
                Err(rr) => {
                    warn!("Couldnt parse token: {}", rr);
                    return;
                }
            },
            Err(rr) => {
                warn!("Couldnt read token file. {}", rr);
                return;
            }
        }
    } else {
        token = Token {
            token: args.get(auth_i.unwrap() + 1).unwrap().to_string(),
            name: args.get(name_i.unwrap() + 1).unwrap().to_string(),
        };
    }

    commands.insert_resource(token);
}
