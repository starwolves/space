use std::collections::HashMap;

use bevy_log::warn;
use serde::Deserialize;

use crate::space::core::networking::resources::{ConsoleCommandVariantValues};

#[derive(Deserialize)]
pub struct Property {
    pub value_type : i64,
    pub value : String,
    pub key : String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExportDataRaw {
    pub properties : Vec<Property>,
}

pub struct ExportData {
    pub properties : HashMap<String, ConsoleCommandVariantValues>,
}

impl ExportData {
    pub fn new(raw: ExportDataRaw) -> ExportData {
        let mut hashmap = HashMap::new();
        for property in raw.properties {
            let v;
            if property.value_type == 4 {
                v = ConsoleCommandVariantValues::String(property.value)
            } else {
                warn!("Entity from entities.json had unknown type!");
                continue;
            }
            hashmap.insert(property.key, v);

        }
        ExportData {
            properties: hashmap,
        }
    }
}
