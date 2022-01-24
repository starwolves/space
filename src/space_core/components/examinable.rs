use std::collections::BTreeMap;

use bevy::prelude::Component;

#[derive(Component)]
pub struct Examinable {
    pub assigned_texts : BTreeMap<u32, String>,
    pub name : RichName,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            assigned_texts : BTreeMap::new(),
            name: RichName::default(),
        }
    }
}

impl Default for RichName {
    fn default() -> Self {
        Self {
            name : "".to_string(),
            n : false,
            the : false,
        }
    }
}


#[derive(Clone, Debug)]
pub struct RichName {
    pub name : String,
    pub n : bool,
    pub the : bool,
}

impl RichName {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_a_name(&self) -> String {
        let prefix;
        if self.the {
            prefix = "the";
        } else {
            if self.n {
                prefix = "an";
            } else {
                prefix = "a";
            }
        }
        prefix.to_owned() + " " + &self.name
    }
}
