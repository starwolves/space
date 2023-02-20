use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use combat::melee_queries::melee_attack_handler;
use combat::sfx::{attack_sfx, health_combat_hit_result_sfx};
use entity::base_mesh::link_base_mesh;
use entity::entity_types::register_entity_type;
use entity::loading::load_entity;
use entity::spawn::build_base_entities;

use hud::inventory::build::InventoryHudState;
use hud::inventory::items::update_inventory_hud_add_item_to_slot;
use hud::inventory::slots::InventoryHudLabels;
use inventory::server::inventory::SpawnItemLabel;
use inventory::spawn_item::build_inventory_items;
use iyes_loopless::prelude::IntoConditionalSystem;
use physics::spawn::build_rigid_bodies;
use resources::is_server::is_server;
use resources::labels::{ActionsLabels, BuildingLabels, CombatLabels, UpdateLabels};

use crate::action::{
    build_actions, construct_action_prequisite_check, construction_tool_actions,
    construction_tool_search_distance_prequisite_check, deconstruct_action_prequisite_check,
    text_tree_input_selection,
};
use crate::construction_tool::{ConstructionTool, InputConstructionOptionsSelection};

use super::{
    construction_tool::{InputConstruct, InputConstructionOptions, InputDeconstruct},
    spawn::{build_construction_tools, ConstructionToolType},
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
        } else {
            app.add_system(
                update_inventory_hud_add_item_to_slot::<ConstructionToolType>
                    .run_if_resource_exists::<InventoryHudState>()
                    .after(InventoryHudLabels::UpdateSlot)
                    .label(InventoryHudLabels::QueueUpdate),
            )
            .add_system(load_entity::<ConstructionToolType>)
            .add_system(link_base_mesh::<ConstructionToolType>);
        }
        register_entity_type::<ConstructionToolType>(app);
        register_basic_console_commands_for_type::<ConstructionToolType>(app);
        register_basic_console_commands_for_inventory_item_type::<ConstructionToolType>(app);
        app.add_system(
            build_construction_tools::<ConstructionToolType>.after(BuildingLabels::TriggerBuild),
        )
        .add_system(
            (build_base_entities::<ConstructionToolType>).after(BuildingLabels::TriggerBuild),
        )
        .add_system(
            (build_rigid_bodies::<ConstructionToolType>).after(BuildingLabels::TriggerBuild),
        )
        .add_system(
            (build_inventory_items::<ConstructionToolType>)
                .after(BuildingLabels::TriggerBuild)
                .after(SpawnItemLabel::SpawnHeldItem)
                .label(SpawnItemLabel::AddingComponent),
        );
    }
}
