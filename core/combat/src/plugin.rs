use api::data::PostUpdateLabels;
use api::{combat::ProjectileFOV, data::CombatLabels};
use bevy::app::CoreStage::PostUpdate;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;

use crate::apply_damage::{finalize_apply_damage, ActiveApplyDamage};
use crate::chat::{blanks_chat, hit_query_chat_cells};
use crate::melee_queries::MeleeBlank;
use crate::projectile_queries::ProjectileBlank;
use crate::sfx::health_combat_hit_result_sfx_cells;
use crate::{
    active_attacks::{cache_attacks, ActiveAttackIncrement, ActiveAttacks},
    apply_damage::{start_apply_damage, HealthCombatHitResult},
    attack::{Attack, QueryCombatHitResult},
    chat::NetHitQueryChat,
    melee_queries::{melee_direct, MeleeDirectQuery},
    projectile_queries::{projectile_attack, ProjectileQuery},
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
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
        .add_system(blanks_chat.after(CombatLabels::FinalizeApplyDamage))
        .add_event::<Attack>()
        .add_event::<ProjectileFOV>()
        .add_event::<MeleeDirectQuery>()
        .add_event::<QueryCombatHitResult>()
        .add_event::<ProjectileQuery>()
        .init_resource::<ActiveAttacks>()
        .init_resource::<ActiveAttackIncrement>()
        .add_event::<HealthCombatHitResult>()
        .add_event::<NetHitQueryChat>()
        .add_event::<ProjectileBlank>()
        .add_event::<MeleeBlank>()
        .add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .after(PostUpdateLabels::VisibleChecker)
                .label(PostUpdateLabels::Net)
                .with_system(net_system::<NetHitQueryChat>),
        )
        .add_system(
            cache_attacks
                .after(CombatLabels::RegisterAttacks)
                .before(CombatLabels::Query)
                .label(CombatLabels::CacheAttack),
        )
        .add_system(health_combat_hit_result_sfx_cells.after(CombatLabels::FinalizeApplyDamage))
        .init_resource::<ActiveApplyDamage>();
    }
}
