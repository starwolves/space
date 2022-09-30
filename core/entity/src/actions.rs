use actions::core::{Action, ActionData, BuildingActions};
use bevy::prelude::{Query, ResMut};

use crate::examine::Examinable;

/// Build examine action.
pub(crate) fn build_actions(
    mut building_action_data: ResMut<BuildingActions>,
    examinable_items: Query<&Examinable>,
) {
    for building_action in building_action_data.list.iter_mut() {
        let mut new_vec = vec![ActionData {
            data: Action {
                id: "actions::pawn/examine".to_string(),
                text: "Examine".to_string(),
                tab_list_priority: u8::MAX,
            },
            approved: None,
        }];
        match building_action.target_entity_option {
            Some(examined_entity) => match examinable_items.get(examined_entity) {
                Ok(_) => {
                    building_action.actions.append(&mut new_vec);
                }
                Err(_rr) => {}
            },
            None => {
                building_action.actions.append(&mut new_vec);
            }
        }
    }
}
/// How far melee fists attacks can reach.
pub const MELEE_FISTS_REACH: f32 = 1.2;
