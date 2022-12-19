use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use combat::melee_queries::melee_attack_handler;
use combat::sfx::{attack_sfx, health_combat_hit_result_sfx};
use entity::entity_data::initialize_entity_data;
use entity::meta::{EntityDataProperties, EntityDataResource};
use entity::spawn::{summon_base_entity, SpawnEvent};
use inventory::spawn_item::summon_inventory_item;
use physics::spawn::summon_rigid_body;
use resources::is_server::is_server;
use resources::labels::{
    ActionsLabels, CombatLabels, StartupLabels, SummoningLabels, UpdateLabels,
};

use crate::action::{
    build_actions, construct_action_prequisite_check, construction_tool_actions,
    construction_tool_is_holding_item_prequisite_check,
    construction_tool_search_distance_prequisite_check, deconstruct_action_prequisite_check,
    text_tree_input_selection,
};
use crate::construction_tool::{
    ConstructionTool, InputConstructionOptionsSelection, CONSTRUCTION_TOOL_ENTITY_NAME,
};

use super::{
    construction_tool::{
        construction_tool, InputConstruct, InputConstructionOptions, InputDeconstruct,
    },
    spawn::{
        default_summon_construction_tool, summon_construction_tool, summon_raw_construction_tool,
        ConstructionToolSummoner,
    },
};

pub struct ConstructionToolAdminPlugin;

impl Plugin for ConstructionToolAdminPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<InputConstruct>()
                .add_event::<InputDeconstruct>()
                .add_event::<InputConstructionOptions>()
                .add_event::<InputConstructionOptionsSelection>()
                .add_system(
                    construction_tool
                        .after(UpdateLabels::TextTreeInputSelection)
                        .before(UpdateLabels::DeconstructCell),
                )
                .add_startup_system(content_initialization.before(StartupLabels::InitEntities))
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
                )
                .add_system(
                    melee_attack_handler::<ConstructionTool>
                        .label(CombatLabels::WeaponHandler)
                        .after(CombatLabels::CacheAttack),
                )
                .add_system(
                    attack_sfx::<ConstructionTool>
                        .after(CombatLabels::WeaponHandler)
                        .after(CombatLabels::Query),
                )
                .add_system(
                    health_combat_hit_result_sfx::<ConstructionTool>
                        .after(CombatLabels::FinalizeApplyDamage),
                )
                .add_system(
                    construction_tool_is_holding_item_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    construction_tool_search_distance_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    deconstruct_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    construct_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    construction_tool_actions
                        .label(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                )
                .add_system(text_tree_input_selection.label(UpdateLabels::TextTreeInputSelection));
        }
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: CONSTRUCTION_TOOL_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
