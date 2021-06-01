use bevy::{core::Timer, prelude::{Entity, ResMut}};

use crate::space_core::resources::sfx_auto_destroy_timers::SfxAutoDestroyTimers;

pub struct Sfx {
    pub area_mask : u8,
    pub attenuation_filter_cutoff_hz : f32,	
    pub attenuation_filter_db : f32,
    pub attenuation_model : u8,
    pub auto_play : bool,	
    pub bus : String,
    pub doppler_tracking : u8,
    pub emission_angle_degrees : f32,
    pub emission_angle_enabled : bool,
    pub emission_angle_filter_attenuation_db : f32,
    pub max_db : f32,
    pub max_distance : f32,
    pub out_of_range_mode : u8,
    pub pitch_scale : f32,
    pub playing : bool,
    pub stream_paused : bool,
    pub unit_db : f32,
    pub unit_size : f32,
    pub stream_id : String,
    pub play_back_position : f32,
    pub auto_destroy : bool,
    pub sfx_replay : bool,

    pub play_back_duration : f32
}

pub fn sfx_auto_destroy(entity : Entity, sfx_auto_destroy_timers : &mut ResMut<SfxAutoDestroyTimers>, play_back_duration : f32) {

    sfx_auto_destroy_timers.timers.insert(entity, Timer::from_seconds(play_back_duration, false));

}
