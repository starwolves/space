use bevy::log::warn;
use bevy::prelude::{Commands, ResMut, Transform};
use rand::prelude::SliceRandom;
use sfx::{builder::sfx_builder, entity_update::SfxAutoDestroyTimers};

use crate::{
    actions::{
        swing1_sfx::Swing1SfxBundle, swing2_sfx::Swing2SfxBundle, swing3_sfx::Swing3SfxBundle,
        swing4_sfx::Swing4SfxBundle,
    },
    combat::{
        block1_sfx::Block1SfxBundle, block2_sfx::Block2SfxBundle, block3_sfx::Block3SfxBundle,
        laser_light_block1_sfx::LaserLightBlock1Bundle,
        laser_light_block2_sfx::LaserLightBlock2Bundle,
        laser_light_block3_sfx::LaserLightBlock3Bundle,
        laser_light_block4_sfx::LaserLightBlock4Bundle, laser_light_hit1_sfx::LaserLightHit1Bundle,
        laser_light_hit2_sfx::LaserLightHit2Bundle, laser_light_hit3_sfx::LaserLightHit3Bundle,
        laser_light_hit4_sfx::LaserLightHit4Bundle, laser_light_shot1_sfx::LaserLightShot1Bundle,
        laser_light_shot2_sfx::LaserLightShot2Bundle, laser_light_shot3_sfx::LaserLightShot3Bundle,
        laser_light_shot4_sfx::LaserLightShot4Bundle, punch1_sfx::Punch1SfxBundle,
        punch2_sfx::Punch2SfxBundle, punch3_sfx::Punch3SfxBundle, punch4_sfx::Punch4SfxBundle,
    },
};
use bevy::prelude::Entity;

/// Manage auto destroying timers.
pub fn sfx_auto_destroy(
    entity: Entity,
    sfx_auto_destroy_timers: &mut ResMut<SfxAutoDestroyTimers>,
) {
    sfx_auto_destroy_timers.timers.push((entity, 0));
}

/// Combat sfx set.
#[derive(Clone)]
pub struct CombatSoundSet {
    pub default: Vec<CombatSound>,
    pub hit_soft: Vec<CombatSound>,
    pub hit_blocked: Vec<CombatSound>,
}

impl CombatSoundSet {
    pub fn default_laser_projectiles() -> Self {
        Self {
            default: vec![
                CombatSound::LaserLightShot1,
                CombatSound::LaserLightShot2,
                CombatSound::LaserLightShot3,
                CombatSound::LaserLightShot4,
            ],
            hit_soft: vec![
                CombatSound::LaserLightHit1,
                CombatSound::LaserLightHit2,
                CombatSound::LaserLightHit3,
                CombatSound::LaserLightHit4,
            ],
            hit_blocked: vec![
                CombatSound::LaserLightBlock1,
                CombatSound::LaserLightBlock2,
                CombatSound::LaserLightBlock3,
                CombatSound::LaserLightBlock4,
            ],
        }
    }

    pub fn spawn_default_sfx(
        &self,
        commands: &mut Commands,
        transform: Transform,
        auto_destroy_timers: &mut ResMut<SfxAutoDestroyTimers>,
    ) {
        match self.default.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::FistsSwing1 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Swing1SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsSwing2 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Swing2SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsSwing3 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Swing3SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsSwing4 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Swing4SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightShot1 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightShot1Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightShot2 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightShot2Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightShot3 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightShot3Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightShot4 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightShot4Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            _ => (),
        }
    }

    pub fn spawn_hit_sfx(
        &self,
        commands: &mut Commands,
        transform: Transform,
        auto_destroy_timers: &mut ResMut<SfxAutoDestroyTimers>,
    ) {
        match self.hit_soft.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::FistsPunch1 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Punch1SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsPunch2 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Punch2SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsPunch3 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Punch3SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsPunch4 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Punch4SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightHit1 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightHit1Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightHit2 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightHit2Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightHit3 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightHit3Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightHit4 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightHit4Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            _ => (),
        }
    }

    pub fn spawn_hit_blocked(
        &self,
        commands: &mut Commands,
        transform: Transform,
        auto_destroy_timers: &mut ResMut<SfxAutoDestroyTimers>,
    ) {
        match self.hit_blocked.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::FistsBlock1 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Block1SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsBlock2 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Block2SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::FistsBlock3 => {
                let sfx_entity = sfx_builder(commands, transform, Box::new(Block3SfxBundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightBlock1 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightBlock1Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightBlock2 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightBlock2Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightBlock3 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightBlock3Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            CombatSound::LaserLightBlock4 => {
                let sfx_entity =
                    sfx_builder(commands, transform, Box::new(LaserLightBlock4Bundle::new));
                sfx_auto_destroy(sfx_entity, auto_destroy_timers);
            }
            _ => {
                warn!("???");
            }
        }
    }
}

/// Contains all combat sounds.
#[derive(Clone)]
pub enum CombatSound {
    FistsSwing1,
    FistsSwing2,
    FistsSwing3,
    FistsSwing4,
    FistsPunch1,
    FistsPunch2,
    FistsPunch3,
    FistsPunch4,
    FistsBlock1,
    FistsBlock2,
    FistsBlock3,
    LaserLightShot1,
    LaserLightShot2,
    LaserLightShot3,
    LaserLightShot4,
    LaserLightBlock1,
    LaserLightBlock2,
    LaserLightBlock3,
    LaserLightBlock4,
    LaserLightHit1,
    LaserLightHit2,
    LaserLightHit3,
    LaserLightHit4,
}

impl Default for CombatSoundSet {
    fn default() -> Self {
        Self {
            default: vec![
                CombatSound::FistsSwing1,
                CombatSound::FistsSwing2,
                CombatSound::FistsSwing3,
                CombatSound::FistsSwing4,
            ],
            hit_soft: vec![
                CombatSound::FistsPunch1,
                CombatSound::FistsPunch2,
                CombatSound::FistsPunch3,
                CombatSound::FistsPunch4,
            ],
            hit_blocked: vec![
                CombatSound::FistsBlock1,
                CombatSound::FistsBlock2,
                CombatSound::FistsBlock3,
            ],
        }
    }
}
