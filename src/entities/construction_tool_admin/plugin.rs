use bevy::app::CoreStage::PostUpdate;
pub struct ConstructionToolAdminPlugin;

impl Plugin for ConstructionToolAdminPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputConstruct>()
            .add_event::<InputDeconstruct>()
            .add_event::<InputConstructionOptions>()
            .add_event::<NetConstructionTool>()
            .add_event::<InputConstructionOptionsSelection>()
            .add_system(
                construction_tool
                    .after(UpdateLabels::TextTreeInputSelection)
                    .before(UpdateLabels::DeconstructCell),
            )
            .add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(construction_tool_actions.after(TabActionsQueueLabels::TabAction))
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            )
            .add_system(
                summon_construction_tool::<ConstructionToolSummoner>
                    .after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_base_entity::<ConstructionToolSummoner>)
                    .after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_rigid_body::<ConstructionToolSummoner>)
                    .after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_inventory_item::<ConstructionToolSummoner>)
                    .after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_construction_tool).after(SummoningLabels::TriggerSummon))
            .add_event::<SpawnEvent<ConstructionToolSummoner>>()
            .add_system(
                (default_summon_construction_tool)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
            );
    }
}

use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut};

use crate::core::{
    entity::{
        entity_data::{initialize_entity_data, EntityDataProperties, EntityDataResource},
        spawn::{summon_base_entity, SpawnEvent},
    },
    inventory_item::spawn::summon_inventory_item,
    rigid_body::spawn::summon_rigid_body,
    space_plugin::plugin::{PostUpdateLabels, StartupLabels, SummoningLabels, UpdateLabels},
    tab_actions::plugin::TabActionsQueueLabels,
};

use super::{
    action::construction_tool_actions,
    construction_tool::{
        construction_tool, InputConstruct, InputConstructionOptions,
        InputConstructionOptionsSelection, InputDeconstruct,
    },
    net::{net_system, NetConstructionTool},
    spawn::{
        default_summon_construction_tool, summon_construction_tool, summon_raw_construction_tool,
        ConstructionToolSummoner, CONSTRUCTION_TOOL_ENTITY_NAME,
    },
};

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: CONSTRUCTION_TOOL_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
