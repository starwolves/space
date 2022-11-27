use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use resources::labels::SummoningLabels;

use crate::commands::{
    entity_console_commands, inventory_item_console_commands, rcon_console_commands, GiveAllRCON,
};

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
                .add_system(rcon_console_commands)
                .insert_resource::<GiveAllRCON>(GiveAllRCON {
                    give: self.give_all_rcon,
                });
        }
    }
}
