use bevy::log::warn;
use bevy::prelude::{Commands, Transform};
use rand::prelude::SliceRandom;
use sfx::builder::sfx_builder;

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

    pub fn spawn_default_sfx(&self, commands: &mut Commands, transform: Transform) {
        match self.default.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::FistsSwing1 => {
                sfx_builder(commands, transform, Box::new(Swing1SfxBundle::new));
            }
            CombatSound::FistsSwing2 => {
                sfx_builder(commands, transform, Box::new(Swing2SfxBundle::new));
            }
            CombatSound::FistsSwing3 => {
                sfx_builder(commands, transform, Box::new(Swing3SfxBundle::new));
            }
            CombatSound::FistsSwing4 => {
                sfx_builder(commands, transform, Box::new(Swing4SfxBundle::new));
            }
            CombatSound::LaserLightShot1 => {
                sfx_builder(commands, transform, Box::new(LaserLightShot1Bundle::new));
            }
            CombatSound::LaserLightShot2 => {
                sfx_builder(commands, transform, Box::new(LaserLightShot2Bundle::new));
            }
            CombatSound::LaserLightShot3 => {
                sfx_builder(commands, transform, Box::new(LaserLightShot3Bundle::new));
            }
            CombatSound::LaserLightShot4 => {
                sfx_builder(commands, transform, Box::new(LaserLightShot4Bundle::new));
            }
            _ => (),
        }
    }

    pub fn spawn_hit_sfx(&self, commands: &mut Commands, transform: Transform) {
        match self.hit_soft.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::FistsPunch1 => {
                sfx_builder(commands, transform, Box::new(Punch1SfxBundle::new));
            }
            CombatSound::FistsPunch2 => {
                sfx_builder(commands, transform, Box::new(Punch2SfxBundle::new));
            }
            CombatSound::FistsPunch3 => {
                sfx_builder(commands, transform, Box::new(Punch3SfxBundle::new));
            }
            CombatSound::FistsPunch4 => {
                sfx_builder(commands, transform, Box::new(Punch4SfxBundle::new));
            }
            CombatSound::LaserLightHit1 => {
                sfx_builder(commands, transform, Box::new(LaserLightHit1Bundle::new));
            }
            CombatSound::LaserLightHit2 => {
                sfx_builder(commands, transform, Box::new(LaserLightHit2Bundle::new));
            }
            CombatSound::LaserLightHit3 => {
                sfx_builder(commands, transform, Box::new(LaserLightHit3Bundle::new));
            }
            CombatSound::LaserLightHit4 => {
                sfx_builder(commands, transform, Box::new(LaserLightHit4Bundle::new));
            }
            _ => (),
        }
    }

    pub fn spawn_hit_blocked(&self, commands: &mut Commands, transform: Transform) {
        match self.hit_blocked.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::FistsBlock1 => {
                sfx_builder(commands, transform, Box::new(Block1SfxBundle::new));
            }
            CombatSound::FistsBlock2 => {
                sfx_builder(commands, transform, Box::new(Block2SfxBundle::new));
            }
            CombatSound::FistsBlock3 => {
                sfx_builder(commands, transform, Box::new(Block3SfxBundle::new));
            }
            CombatSound::LaserLightBlock1 => {
                sfx_builder(commands, transform, Box::new(LaserLightBlock1Bundle::new));
            }
            CombatSound::LaserLightBlock2 => {
                sfx_builder(commands, transform, Box::new(LaserLightBlock2Bundle::new));
            }
            CombatSound::LaserLightBlock3 => {
                sfx_builder(commands, transform, Box::new(LaserLightBlock3Bundle::new));
            }
            CombatSound::LaserLightBlock4 => {
                sfx_builder(commands, transform, Box::new(LaserLightBlock4Bundle::new));
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
