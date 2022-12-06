use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use controller::networking::InputUIInput;
use iyes_loopless::prelude::IntoConditionalSystem;
use networking::client::is_client_connected;
use player::plugin::ConfigurationLabel;
use resources::labels::SummoningLabels;

use crate::core::{
    client_init_setup_ui, configure, initialize_setupui, new_clients_enable_setupui,
    register_ui_input_boarding, ui_input_boarding, SetupUiState,
};
/// Atmospherics systems ordering label.

pub struct SetupUiPlugin;

impl Plugin for SetupUiPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(register_ui_input_boarding)
                .add_system(ui_input_boarding)
                .add_system(initialize_setupui.label(SummoningLabels::TriggerSummon))
                .add_event::<InputUIInput>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                )
                .add_system(new_clients_enable_setupui)
                .init_resource::<SetupUiState>();
        } else {
            app.add_system(client_init_setup_ui.run_if(is_client_connected));
        }
    }
}
