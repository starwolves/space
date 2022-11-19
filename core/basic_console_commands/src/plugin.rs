use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use networking::server::net_system;
use resources::labels::{PostUpdateLabels, SummoningLabels};

use crate::commands::{
    entity_console_commands, inventory_item_console_commands, rcon_console_commands, GiveAllRCON,
    NetBasicConsoleCommands,
};
use bevy::app::CoreStage::PostUpdate;

#[derive(Default)]
pub struct BasicConsoleCommandsPlugin {
    pub give_all_rcon: bool,
}

impl Plugin for BasicConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(entity_console_commands.after(SummoningLabels::DefaultSummon))
                .add_system(
                    inventory_item_console_commands
                        .before(SummoningLabels::TriggerSummon)
                        .label(SummoningLabels::NormalSummon),
                )
                .add_event::<NetBasicConsoleCommands>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetBasicConsoleCommands>),
                )
                .add_system(rcon_console_commands)
                .insert_resource::<GiveAllRCON>(GiveAllRCON {
                    give: self.give_all_rcon,
                });
        }
    }
}
