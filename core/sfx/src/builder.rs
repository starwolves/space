use bevy::{
    prelude::{Commands, Component, Entity, Transform},
    time::Timer,
};
use const_format::concatcp;
use entity::{
    entity_data::{CachedBroadcastTransform, EntityData, EntityUpdates, UpdateTransform},
    entity_types::EntityType,
    sensable::Sensable,
};
use rand::Rng;
use resources::content::SF_CONTENT_PREFIX;
pub const SFX_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "sfx");

#[derive(Clone)]
pub struct AmbienceSfxEntityType {
    pub identifier: String,
}
impl Default for AmbienceSfxEntityType {
    fn default() -> Self {
        Self {
            identifier: SF_CONTENT_PREFIX.to_string() + "ambience_sfx",
        }
    }
}

impl EntityType for AmbienceSfxEntityType {
    fn to_string(&self) -> String {
        self.identifier.to_string()
    }
    fn is_type(&self, identifier: String) -> bool {
        self.identifier == identifier
    }
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }
}
#[derive(Clone)]
pub struct RepeatingSfxEntityType {
    pub identifier: String,
}
impl Default for RepeatingSfxEntityType {
    fn default() -> Self {
        Self {
            identifier: SF_CONTENT_PREFIX.to_string() + "repeating_sfx",
        }
    }
}

impl EntityType for RepeatingSfxEntityType {
    fn to_string(&self) -> String {
        self.identifier.clone()
    }
    fn is_type(&self, identifier: String) -> bool {
        self.identifier == identifier
    }
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }
}
#[derive(Clone)]
pub struct SfxEntityType {
    pub identifier: String,
}

impl Default for SfxEntityType {
    fn default() -> Self {
        Self {
            identifier: SF_CONTENT_PREFIX.to_string() + "sfx",
        }
    }
}

impl EntityType for SfxEntityType {
    fn to_string(&self) -> String {
        SFX_ENTITY_NAME.to_string()
    }
    fn is_type(&self, identifier: String) -> bool {
        self.identifier == identifier
    }
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }
}
use entity::entity_data::EntityGroup;

/// Spawn background sound effect with commands as a function.
#[cfg(feature = "server")]
pub fn spawn_ambience_sfx(
    commands: &mut Commands,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(&mut Commands) -> Entity + Sync + Send>,
) -> Entity {
    let entity = (builder)(commands);

    commands.entity(entity).insert((
        rigid_body_position,
        EntityData {
            entity_type: Box::new(AmbienceSfxEntityType::new()),
            entity_group: EntityGroup::default(),
        },
        Sensable {
            is_audible: true,
            always_sensed: true,
            ..Default::default()
        },
        EntityUpdates::default(),
    ));

    entity
}

/// Component for repeating sfx for sprinting footsteps.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct FootstepsSprinting;

/// Component for repeating sfx for walking footsteps.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct FootstepsWalking;

/// Component for repeating sfx.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct RepeatingSfx {
    pub area_mask: u8,
    pub attenuation_filter_cutoff_hz: f32,
    pub attenuation_filter_db: f32,
    pub attenuation_model: u8,
    pub auto_play: bool,
    pub bus: String,
    pub doppler_tracking: u8,
    pub emission_angle_degrees: f32,
    pub emission_angle_enabled: bool,
    pub emission_angle_filter_attenuation_db: f32,
    pub max_db: f32,
    pub max_distance: f32,
    pub out_of_range_mode: u8,
    pub pitch_scale: f32,
    pub playing: bool,
    pub stream_paused: bool,
    pub unit_db: f32,
    pub unit_size: f32,
    pub stream_id: String,
    pub auto_destroy: bool,
    pub repeat_time: f32,
}

#[cfg(feature = "server")]
impl Default for RepeatingSfx {
    fn default() -> Self {
        Self {
            area_mask: 0,
            attenuation_filter_cutoff_hz: 5000.,
            attenuation_filter_db: -24.,
            attenuation_model: 0,
            auto_play: true,
            bus: "Master".to_string(),
            doppler_tracking: 0,
            emission_angle_degrees: 45.,
            emission_angle_enabled: false,
            emission_angle_filter_attenuation_db: -12.,
            max_db: 3.,
            max_distance: 0.,
            out_of_range_mode: 0,
            pitch_scale: 1.,
            playing: false,
            stream_paused: false,
            unit_db: 0.,
            unit_size: 1.,
            stream_id: "".to_string(),
            auto_destroy: true,
            repeat_time: 1.,
        }
    }
}

/// Component for SFX.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Sfx {
    pub area_mask: u8,
    pub attenuation_filter_cutoff_hz: f32,
    pub attenuation_filter_db: f32,
    pub attenuation_model: u8,
    pub auto_play: bool,
    pub bus: String,
    pub doppler_tracking: u8,
    pub emission_angle_degrees: f32,
    pub emission_angle_enabled: bool,
    pub emission_angle_filter_attenuation_db: f32,
    pub max_db: f32,
    pub max_distance: f32,
    pub out_of_range_mode: u8,
    pub pitch_scale: f32,
    pub playing: bool,
    pub stream_paused: bool,
    pub unit_db: f32,
    pub unit_size: f32,
    pub stream_id: String,
    pub play_back_position: f32,
    pub auto_destroy: bool,
    pub sfx_replay: bool,
    pub play_back_duration: f32,
}

/// Get an acceptable randomly variating pitch.
#[cfg(feature = "server")]
pub fn get_random_pitch_scale(input_scale: f32) -> f32 {
    let mut rng = rand::thread_rng();
    input_scale + rng.gen_range(-0.2..0.2)
}

#[cfg(feature = "server")]
impl Default for Sfx {
    fn default() -> Self {
        Self {
            area_mask: 0,
            attenuation_filter_cutoff_hz: 5000.,
            attenuation_filter_db: -24.,
            attenuation_model: 0,
            auto_play: true,
            bus: "Master".to_string(),
            doppler_tracking: 0,
            emission_angle_degrees: 45.,
            emission_angle_enabled: false,
            emission_angle_filter_attenuation_db: -12.,
            max_db: 3.,
            max_distance: 0.,
            out_of_range_mode: 0,
            pitch_scale: 1.,
            playing: false,
            stream_paused: false,
            unit_db: 0.,
            unit_size: 1.,
            stream_id: "".to_string(),
            play_back_position: 0.,
            auto_destroy: true,
            sfx_replay: false,
            play_back_duration: 3.5,
        }
    }
}

/// Replay timer for ambience SFX.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct AmbienceSfxTimer {
    pub timer: Timer,
}

/// Function that spawns a repeating sound effect with commands.
#[cfg(feature = "server")]
pub fn repeating_sfx_builder(
    commands: &mut Commands,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(&mut Commands) -> Entity + Sync + Send>,
) -> Entity {
    let entity = (builder)(commands);
    commands.entity(entity).insert((
        rigid_body_position,
        EntityData {
            entity_type: Box::new(RepeatingSfxEntityType::new()),
            entity_group: EntityGroup::default(),
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        EntityUpdates::default(),
        UpdateTransform,
        CachedBroadcastTransform::default(),
    ));
    entity
}
/// Function that builds a sound effect.
#[cfg(feature = "server")]
pub fn sfx_builder(
    commands: &mut Commands,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(&mut Commands) -> Entity + Sync + Send>,
) -> Entity {
    let entity = (builder)(commands);
    commands.entity(entity).insert((
        rigid_body_position,
        EntityData {
            entity_type: Box::new(SfxEntityType::new()),
            entity_group: EntityGroup::default(),
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        EntityUpdates::default(),
    ));
    entity
}
