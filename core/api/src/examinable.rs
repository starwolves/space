use std::collections::BTreeMap;

use bevy::prelude::{Component, Entity, SystemLabel};

use crate::chat::ASTRIX;

#[derive(Component, Default)]
pub struct Examinable {
    pub assigned_texts: BTreeMap<u32, String>,
    pub name: RichName,
}

#[derive(Clone, Debug)]
pub struct RichName {
    pub name: String,
    pub n: bool,
    pub the: bool,
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

impl Default for RichName {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            n: false,
            the: false,
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ExamineLabels {
    Start,
    Default,
}
pub struct InputExamineEntity {
    pub handle: u64,
    pub examine_entity: Entity,
    pub entity: Entity,
    pub message: String,
}
impl Default for InputExamineEntity {
    fn default() -> Self {
        Self {
            handle: 0,
            examine_entity: Entity::from_bits(0),
            entity: Entity::from_bits(0),
            message: ASTRIX.to_string(),
        }
    }
}
