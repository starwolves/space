use bevy::{core::Timer, prelude::{Entity, ResMut}};
use rand::Rng;

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

    sfx_auto_destroy_timers.timers.push((entity, Timer::from_seconds(play_back_duration, false)));

}

pub fn get_random_pitch_scale(
    input_scale : f32,
) -> f32 {
    let mut rng = rand::thread_rng();
    input_scale + rng.gen_range(-0.2..0.2)
}

impl Default for Sfx {
    fn default() -> Self {
        Self {
            area_mask : 0,
            attenuation_filter_cutoff_hz: 5000.,
            attenuation_filter_db : -24.,
            attenuation_model: 0,
            auto_play : true,
            bus : "Master".to_string(),
            doppler_tracking : 0,
            emission_angle_degrees : 45.,
            emission_angle_enabled: false,
            emission_angle_filter_attenuation_db : -12.,
            max_db : 3.,
            max_distance: 0.,
            out_of_range_mode : 0,
            pitch_scale: 1.,
            playing : false,
            stream_paused : false,
            unit_db: 0.,
            unit_size: 1.,
            stream_id: "".to_string(),
            play_back_position: 0.,
            auto_destroy : true,
            sfx_replay :false,
            play_back_duration: 3.5,
        }
    }
}
