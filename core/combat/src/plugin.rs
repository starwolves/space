use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use resources::modes::is_server_mode;
use resources::ordering::{CombatSet, Update};

use crate::apply_damage::{finalize_apply_damage, ActiveApplyDamage};
use crate::chat::hit_query_chat_cells;
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
        if is_server_mode(app) {
            app.add_systems(
                Update,
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
                ),
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
                Update,
                (
                    cache_attacks
                        .after(CombatSet::RegisterAttacks)
                        .before(CombatSet::Query)
                        .in_set(CombatSet::CacheAttack),
                    health_combat_hit_result_sfx_cells.after(CombatSet::FinalizeApplyDamage),
                ),
            )
            .init_resource::<ActiveApplyDamage>();
        }
    }
}
