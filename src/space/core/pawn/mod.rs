use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

use self::{
    actions::pawn_actions,
    resources::{AuthidI, UsedNames},
    systems::{toggle_combat_mode::toggle_combat_mode, user_name::user_name},
};

use super::tab_actions::TabActionsQueueLabels;

pub mod actions;
pub mod components;
pub mod functions;
pub mod resources;
pub mod systems;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AuthidI>()
            .init_resource::<UsedNames>()
            .add_system(user_name)
            .add_system(toggle_combat_mode)
            .add_system(pawn_actions.after(TabActionsQueueLabels::TabAction));
    }
}
