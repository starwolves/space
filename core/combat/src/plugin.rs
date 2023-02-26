use bevy::app::CoreStage::PostUpdate;
use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use resources::is_server::is_server;
use resources::labels::{CombatLabels, PostUpdateLabels};

use crate::apply_damage::{finalize_apply_damage, ActiveApplyDamage};
use crate::chat::hit_query_chat_cells;
use crate::health_ui::{health_ui_update, ClientHealthUICache};
use crate::melee_queries::MeleeBlank;
use crate::projectile_queries::ProjectileBlank;
use crate::sfx::health_combat_hit_result_sfx_cells;
use crate::{
    active_attacks::{cache_attacks, ActiveAttackIncrement, ActiveAttacks},
    apply_damage::{start_apply_damage, HealthCombatHitResult},
    attack::{Attack, QueryCombatHitResult},
    melee_queries::{melee_direct, MeleeDirectQuery},
    projectile_queries::{projectile_attack, ProjectileQuery},
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                melee_direct
                    .after(CombatLabels::WeaponHandler)
                    .label(CombatLabels::Query),
            )
            .add_system(
                projectile_attack
                    .after(CombatLabels::WeaponHandler)
                    .label(CombatLabels::Query),
            )
            .add_system(
                start_apply_damage
                    .label(CombatLabels::StartApplyDamage)
                    .before(CombatLabels::FinalizeApplyDamage)
                    .after(CombatLabels::Query),
            )
            .add_system(
                finalize_apply_damage
                    .label(CombatLabels::FinalizeApplyDamage)
                    .after(CombatLabels::StartApplyDamage)
                    .after(CombatLabels::Query),
            )
            .add_system(hit_query_chat_cells.after(CombatLabels::FinalizeApplyDamage))
            /*.add_system(
                blanks_chat
                    .after(CombatLabels::FinalizeApplyDamage)
                    .after(EntityProximityMessages::Send),
            )*/
            .add_event::<Attack>()
            .add_event::<MeleeDirectQuery>()
            .add_event::<QueryCombatHitResult>()
            .add_event::<ProjectileQuery>()
            .init_resource::<ActiveAttacks>()
            .init_resource::<ActiveAttackIncrement>()
            .add_event::<HealthCombatHitResult>()
            .add_event::<ProjectileBlank>()
            .add_event::<MeleeBlank>()
            .add_system(
                cache_attacks
                    .after(CombatLabels::RegisterAttacks)
                    .before(CombatLabels::Query)
                    .label(CombatLabels::CacheAttack),
            )
            .add_system(health_combat_hit_result_sfx_cells.after(CombatLabels::FinalizeApplyDamage))
            .init_resource::<ActiveApplyDamage>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(health_ui_update),
            )
            .init_resource::<ClientHealthUICache>();
        }
    }
}
