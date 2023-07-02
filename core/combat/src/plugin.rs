use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
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
    melee_queries::MeleeDirectQuery,
    projectile_queries::ProjectileQuery,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            /*app.add_system(
                melee_direct
                    .after(CombatLabels::WeaponHandler)
                    .in_set(CombatLabels::Query),
            )
            .add_system(
                projectile_attack
                    .after(CombatLabels::WeaponHandler)
                    .in_set(CombatLabels::Query),
            )*/
            app.add_system(
                start_apply_damage
                    .in_set(CombatLabels::StartApplyDamage)
                    .before(CombatLabels::FinalizeApplyDamage)
                    .after(CombatLabels::Query),
            )
            .add_system(
                finalize_apply_damage
                    .in_set(CombatLabels::FinalizeApplyDamage)
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
                    .in_set(CombatLabels::CacheAttack),
            )
            .add_system(health_combat_hit_result_sfx_cells.after(CombatLabels::FinalizeApplyDamage))
            .init_resource::<ActiveApplyDamage>()
            .add_system(
                health_ui_update
                    .in_base_set(CoreSet::PostUpdate)
                    .in_set(PostUpdateLabels::EntityUpdate),
            )
            .init_resource::<ClientHealthUICache>();
        }
    }
}
