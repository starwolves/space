use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use controller::networking::InputUIInput;
use networking::server::net_system;
use player::plugin::ConfigurationLabel;
use resources::labels::{PostUpdateLabels, SummoningLabels};

use crate::core::{
    configure, initialize_setupui, new_clients_enable_setupui, register_ui_input_boarding,
    ui_input_boarding, NetConfigure, NetOnSetupUI, NetUIInputTransmitData, SetupUiState,
};
use bevy::app::CoreStage::PostUpdate;
/// Atmospherics systems ordering label.

pub struct SetupUiPlugin;

impl Plugin for SetupUiPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(register_ui_input_boarding)
                .add_system(ui_input_boarding)
                .add_system(initialize_setupui.label(SummoningLabels::TriggerSummon))
                .add_event::<NetUIInputTransmitData>()
                .add_event::<InputUIInput>()
                .add_event::<NetOnSetupUI>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetUIInputTransmitData>)
                        .with_system(net_system::<NetOnSetupUI>)
                        .with_system(net_system::<NetConfigure>),
                )
                .add_event::<NetConfigure>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                )
                .add_system(new_clients_enable_setupui)
                .init_resource::<SetupUiState>();
        }
    }
}
