use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};

use crate::core::tab_actions::plugin::TabActionsQueueLabels;

use super::{
    actions::actions,
    toggle_combat_mode::toggle_combat_mode,
    user_name::{user_name, AuthidI, UsedNames},
};

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AuthidI>()
            .init_resource::<UsedNames>()
            .add_system(user_name)
            .add_system(toggle_combat_mode)
            .add_system(actions.after(TabActionsQueueLabels::TabAction));
    }
}
