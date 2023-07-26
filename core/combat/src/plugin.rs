use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use resources::is_server::is_server;
use resources::sets::{CombatSet, MainSet, PostUpdateSet};

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
            app.add_systems(
                FixedUpdate,
                (
                    start_apply_damage
                        .in_set(CombatSet::StartApplyDamage)
                        .before(CombatSet::FinalizeApplyDamage)
                        .after(CombatSet::Query),
                    finalize_apply_damage
                        .in_set(CombatSet::FinalizeApplyDamage)
                        .after(CombatSet::StartApplyDamage)
                        .after(CombatSet::Query),
                    hit_query_chat_cells.after(CombatSet::FinalizeApplyDamage),
                )
                    .in_set(MainSet::Update),
            )
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
            .add_systems(
                FixedUpdate,
                (
                    cache_attacks
                        .after(CombatSet::RegisterAttacks)
                        .before(CombatSet::Query)
                        .in_set(CombatSet::CacheAttack),
                    health_combat_hit_result_sfx_cells.after(CombatSet::FinalizeApplyDamage),
                )
                    .in_set(MainSet::Update),
            )
            .init_resource::<ActiveApplyDamage>()
            .add_systems(
                FixedUpdate,
                health_ui_update
                    .in_set(PostUpdateSet::EntityUpdate)
                    .in_set(MainSet::PostUpdate),
            )
            .init_resource::<ClientHealthUICache>();
        }
    }
}
