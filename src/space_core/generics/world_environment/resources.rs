
use bevy::prelude::{Color, FromWorld, ResMut, World};
use bevy::math::Quat;
use serde::{Serialize, Deserialize};

use crate::space_core::generics::entity::functions::string_to_type_converters::{string_color_to_color, string_quat_to_quat};

// The resource we send to each client on connection to initialize worldEnvironment.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WorldEnvironment {
    pub adjustment_brightness: f32,
    pub adjustment_contrast: f32,
    pub adjustment_enabled: bool,
    pub adjustment_saturation : f32,
    pub ambient_light_color : Color,
    pub ambient_light_energy : f32,
    pub ambient_light_sky_contribution : f32,
    pub tonemap_auto_exposure : bool,
    pub tonemap_auto_exposure_max : f32,
    pub tonemap_auto_exposure_min : f32,
    pub tonemap_auto_exposure_grey : f32,
    pub tonemap_auto_exposure_speed : f32,
    pub camera_feed_id : i64,
    pub canvas_max_layer : i64,
    pub bg_color : Color,
    pub bg_energy : f32,
    pub background_mode : u8,
    pub sky_custom_fov : f32,
    pub sky_orientation : Quat,
    pub dof_blur_far_amount : f32,
    pub dof_blur_far_distance : f32,
    pub dof_blur_far_enabled : bool,
    pub dof_blur_far_quality : u8,
    pub dof_blur_far_transition: f32,
    pub dof_blur_near_amount: f32,
    pub dof_blur_near_distance: f32,
    pub dof_blur_near_enabled : bool,
    pub dof_blur_near_quality : f32,
    pub dof_blur_near_transition : f32,
    pub fog_color : Color,
    pub fog_depth_begin : f32,
    pub fog_depth_curve : f32,
    pub fog_depth_enabled : bool,
    pub fog_depth_end : f32,
    pub fog_enabled : bool,
    pub fog_height_curve : f32,
    pub fog_height_enabled : bool,
    pub fog_height_max : f32,
    pub fog_height_min : f32,
    pub fog_sun_amount : f32,
    pub fog_sun_color : Color,
    pub fog_transmit_curve : f32,
    pub fog_transmit_enabled : bool,
    pub glow_bicubic_upscale_enabled : bool,
    pub glow_blend_mode : u8,
    pub glow_bloom : f32,
    pub glow_enabled : bool,
    pub glow_hdr_luminance_cap : f32,
    pub glow_hdr_bleed_scale : f32,
    pub glow_hdr_bleed_treshold  : f32,
    pub glow_intensity : f32,
    pub glow_strength : f32,
    pub ssr_depth_tolerance : f32,
    pub ssr_enabled : bool,
    pub ssr_fade_in : f32,
    pub ssr_fade_out : f32,
    pub ssr_max_steps : i64,
    pub ssr_rough : bool,
    pub ssao_ao_channel_affect : f32,
    pub ssao_bias : f32,
    pub ssao_blur : u8,
    pub ssao_color : Color,
    pub ssao_edge_sharpness : f32,
    pub ssao_enabled : bool,
    pub ssao_intensity : f32,
    pub ssao_intensity2 : f32,
    pub ssao_direct_light_affect : f32,
    pub ssao_quality : u8,
    pub ssao_radius : f32,
    pub ssao_radius2 : f32,
    pub tone_map_exposure : f32,
    pub tone_mapper : u8,
    pub tone_map_white : f32,
    pub glow_high_quality : bool,
}

impl WorldEnvironment {

    pub fn adjust(self, map_environment : &mut ResMut<WorldEnvironment>) {
        map_environment.adjustment_brightness= self.adjustment_brightness;
        map_environment.adjustment_contrast= self.adjustment_contrast;
        map_environment.adjustment_enabled= self.adjustment_enabled;
        map_environment.adjustment_saturation= self.adjustment_saturation;
        map_environment.ambient_light_color= self.ambient_light_color;
        map_environment.ambient_light_energy= self.ambient_light_energy;
        map_environment.ambient_light_sky_contribution= self.ambient_light_sky_contribution;
        map_environment.tonemap_auto_exposure= self.tonemap_auto_exposure;
        map_environment.tonemap_auto_exposure_max= self.tonemap_auto_exposure_max;
        map_environment.tonemap_auto_exposure_min= self.tonemap_auto_exposure_min;
        map_environment.tonemap_auto_exposure_grey= self.tonemap_auto_exposure_grey;
        map_environment.tonemap_auto_exposure_speed= self.tonemap_auto_exposure_speed;
        map_environment.camera_feed_id= self.camera_feed_id;
        map_environment.canvas_max_layer= self.canvas_max_layer;
        map_environment.bg_color= self.bg_color;
        map_environment.bg_energy=self.bg_energy;
        map_environment.background_mode= self.background_mode;
        map_environment.sky_custom_fov=self.sky_custom_fov;
        map_environment.sky_orientation=self.sky_orientation;
        map_environment.dof_blur_far_amount= self.dof_blur_far_amount;
        map_environment.dof_blur_far_distance= self.dof_blur_far_distance;
        map_environment.dof_blur_far_enabled= self.dof_blur_far_enabled;
        map_environment.dof_blur_far_quality= self.dof_blur_far_quality;
        map_environment.dof_blur_far_transition= self.dof_blur_far_transition;
        map_environment.dof_blur_near_amount= self.dof_blur_near_amount;
        map_environment.dof_blur_near_distance= self.dof_blur_near_distance;
        map_environment.dof_blur_near_enabled= self.dof_blur_near_enabled;
        map_environment.dof_blur_near_quality= self.dof_blur_near_quality;
        map_environment.dof_blur_near_transition= self.dof_blur_near_transition;
        map_environment.fog_color=self.fog_color;
        map_environment.fog_depth_begin= self.fog_depth_begin;
        map_environment.fog_depth_curve= self.fog_depth_curve;
        map_environment.fog_depth_enabled= self.fog_depth_enabled;
        map_environment.fog_depth_end= self.fog_depth_end;
        map_environment.fog_enabled=self.fog_enabled;
        map_environment.fog_height_curve= self.fog_height_curve;
        map_environment.fog_height_enabled= self.fog_height_enabled;
        map_environment.fog_height_max= self.fog_height_max;
        map_environment.fog_height_min= self.fog_height_min;
        map_environment.fog_sun_amount= self.fog_sun_amount;
        map_environment.fog_sun_color= self.fog_sun_color;
        map_environment.fog_transmit_curve= self.fog_transmit_curve;
        map_environment.fog_transmit_enabled= self.fog_transmit_enabled;
        map_environment.glow_bicubic_upscale_enabled= self.glow_bicubic_upscale_enabled;
        map_environment.glow_blend_mode= self.glow_blend_mode;
        map_environment.glow_bloom=self.glow_bloom;
        map_environment.glow_enabled= self.glow_enabled;
        map_environment.glow_hdr_luminance_cap= self.glow_hdr_luminance_cap;
        map_environment.glow_hdr_bleed_scale= self.glow_hdr_bleed_scale;
        map_environment.glow_hdr_bleed_treshold= self.glow_hdr_bleed_treshold;
        map_environment.glow_intensity=self.glow_intensity;
        map_environment.glow_strength= self.glow_strength;
        map_environment.ssr_depth_tolerance= self.ssr_depth_tolerance;
        map_environment.ssr_enabled=self.ssr_enabled;
        map_environment.ssr_fade_in= self.ssr_fade_in;
        map_environment.ssr_fade_out= self.ssr_fade_out;
        map_environment.ssr_max_steps= self.ssr_max_steps;
        map_environment.ssr_rough= self.ssr_rough;
        map_environment.ssao_ao_channel_affect= self.ssao_ao_channel_affect;
        map_environment.ssao_bias= self.ssao_bias;
        map_environment.ssao_blur= self.ssao_blur;
        map_environment.ssao_color= self.ssao_color;
        map_environment.ssao_edge_sharpness= self.ssao_edge_sharpness;
        map_environment.ssao_enabled= self.ssao_enabled;
        map_environment.ssao_intensity= self.ssao_intensity;
        map_environment.ssao_intensity2=self.ssao_intensity2;
        map_environment.ssao_direct_light_affect=self.ssao_direct_light_affect;
        map_environment.ssao_quality= self.ssao_quality;
        map_environment.ssao_radius= self.ssao_radius;
        map_environment.ssao_radius2= self.ssao_radius2;
        map_environment.tone_map_exposure= self.tone_map_exposure;
        map_environment.tone_mapper=self.tone_mapper;
        map_environment.tone_map_white= self.tone_map_white;
        map_environment.glow_high_quality= self.glow_high_quality;
    }

    pub fn new(raw : WorldEnvironmentRaw) -> WorldEnvironment {


        WorldEnvironment {
            adjustment_brightness: raw.adjustment_brightness,
            adjustment_contrast: raw.adjustment_contrast,
            adjustment_enabled: raw.adjustment_enabled,
            adjustment_saturation : raw.adjustment_saturation,
            ambient_light_color : string_color_to_color(&raw.ambient_light_color),
            ambient_light_energy : raw.ambient_light_energy,
            ambient_light_sky_contribution : raw.ambient_light_sky_contribution,
            tonemap_auto_exposure : raw.tonemap_auto_exposure,
            tonemap_auto_exposure_max : raw.tonemap_auto_exposure_max,
            tonemap_auto_exposure_min : raw.tonemap_auto_exposure_min,
            tonemap_auto_exposure_grey : raw.tonemap_auto_exposure_grey,
            tonemap_auto_exposure_speed : raw.tonemap_auto_exposure_speed,
            camera_feed_id : raw.camera_feed_id,
            canvas_max_layer : raw.canvas_max_layer,
            bg_color : string_color_to_color(&raw.bg_color),
            bg_energy : raw.bg_energy,
            background_mode : raw.background_mode,
            sky_custom_fov : raw.sky_custom_fov,
            sky_orientation : string_quat_to_quat(&raw.sky_orientation),
            dof_blur_far_amount : raw.dof_blur_far_amount,
            dof_blur_far_distance : raw.dof_blur_far_distance,
            dof_blur_far_enabled : raw.dof_blur_far_enabled,
            dof_blur_far_quality : raw.dof_blur_far_quality,
            dof_blur_far_transition: raw.dof_blur_far_transition,
            dof_blur_near_amount: raw.dof_blur_near_amount,
            dof_blur_near_distance: raw.dof_blur_near_distance,
            dof_blur_near_enabled : raw.dof_blur_near_enabled,
            dof_blur_near_quality : raw.dof_blur_near_quality,
            dof_blur_near_transition : raw.dof_blur_near_transition,
            fog_color : string_color_to_color(&raw.fog_color),
            fog_depth_begin : raw.fog_depth_begin,
            fog_depth_curve : raw.fog_depth_curve,
            fog_depth_enabled : raw.fog_depth_enabled,
            fog_depth_end : raw.fog_depth_end,
            fog_enabled : raw.fog_enabled,
            fog_height_curve : raw.fog_height_curve,
            fog_height_enabled : raw.fog_height_enabled,
            fog_height_max : raw.fog_height_max,
            fog_height_min : raw.fog_height_min,
            fog_sun_amount : raw.fog_sun_amount,
            fog_sun_color : string_color_to_color(&raw.fog_sun_color),
            fog_transmit_curve : raw.fog_transmit_curve,
            fog_transmit_enabled : raw.fog_transmit_enabled,
            glow_bicubic_upscale_enabled : raw.glow_bicubic_upscale_enabled,
            glow_blend_mode : raw.glow_blend_mode,
            glow_bloom : raw.glow_bloom,
            glow_enabled : raw.glow_enabled,
            glow_hdr_luminance_cap : raw.glow_hdr_luminance_cap,
            glow_hdr_bleed_scale : raw.glow_hdr_bleed_scale,
            glow_hdr_bleed_treshold  : raw.glow_hdr_bleed_treshold,
            glow_intensity : raw.glow_intensity,
            glow_strength : raw.glow_strength,
            ssr_depth_tolerance : raw.ssr_depth_tolerance,
            ssr_enabled : raw.ssr_enabled,
            ssr_fade_in : raw.ssr_fade_in,
            ssr_fade_out : raw.ssr_fade_out,
            ssr_max_steps : raw.ssr_max_steps,
            ssr_rough : raw.ssr_rough,
            ssao_ao_channel_affect : raw.ssao_ao_channel_affect,
            ssao_bias : raw.ssao_bias,
            ssao_blur : raw.ssao_blur,
            ssao_color : string_color_to_color(&raw.ssao_color),
            ssao_edge_sharpness : raw.ssao_edge_sharpness,
            ssao_enabled : raw.ssao_enabled,
            ssao_intensity : raw.ssao_intensity,
            ssao_intensity2 : raw.ssao_intensity2,
            ssao_direct_light_affect : raw.ssao_direct_light_affect,
            ssao_quality : raw.ssao_quality,
            ssao_radius : raw.ssao_radius,
            ssao_radius2 : raw.ssao_radius2,
            tone_map_exposure : raw.tone_map_exposure,
            tone_mapper : raw.tone_mapper,
            tone_map_white : raw.tone_map_white,
            glow_high_quality : raw.glow_high_quality,
        }


    }

}



// Each struct value how it gets interpreted by verde-json and how it gets exported via the official JSON.print method on the godot server.
#[derive(Deserialize)]
pub struct WorldEnvironmentRaw {
    adjustment_brightness: f32,
    adjustment_contrast: f32,
    adjustment_enabled: bool,
    adjustment_saturation : f32,
    ambient_light_color : String,
    ambient_light_energy : f32,
    ambient_light_sky_contribution : f32,
    tonemap_auto_exposure : bool,
    tonemap_auto_exposure_max : f32,
    tonemap_auto_exposure_min : f32,
    tonemap_auto_exposure_grey : f32,
    tonemap_auto_exposure_speed : f32,
    camera_feed_id : i64,
    canvas_max_layer : i64,
    bg_color : String,
    bg_energy : f32,
    background_mode : u8,
    sky_custom_fov : f32,
    sky_orientation : String,
    dof_blur_far_amount : f32,
    dof_blur_far_distance : f32,
    dof_blur_far_enabled : bool,
    dof_blur_far_quality : u8,
    dof_blur_far_transition: f32,
    dof_blur_near_amount: f32,
    dof_blur_near_distance: f32,
    dof_blur_near_enabled : bool,
    dof_blur_near_quality : f32,
    dof_blur_near_transition : f32,
    fog_color : String,
    fog_depth_begin : f32,
    fog_depth_curve : f32,
    fog_depth_enabled : bool,
    fog_depth_end : f32,
    fog_enabled : bool,
    fog_height_curve : f32,
    fog_height_enabled : bool,
    fog_height_max : f32,
    fog_height_min : f32,
    fog_sun_amount : f32,
    fog_sun_color : String,
    fog_transmit_curve : f32,
    fog_transmit_enabled : bool,
    glow_bicubic_upscale_enabled : bool,
    glow_blend_mode : u8,
    glow_bloom : f32,
    glow_enabled : bool,
    glow_hdr_luminance_cap : f32,
    glow_hdr_bleed_scale : f32,
    glow_hdr_bleed_treshold  : f32,
    glow_intensity : f32,
    glow_strength : f32,
    ssr_depth_tolerance : f32,
    ssr_enabled : bool,
    ssr_fade_in : f32,
    ssr_fade_out : f32,
    ssr_max_steps : i64,
    ssr_rough : bool,
    ssao_ao_channel_affect : f32,
    ssao_bias : f32,
    ssao_blur : u8,
    ssao_color : String,
    ssao_edge_sharpness : f32,
    ssao_enabled : bool,
    ssao_intensity : f32,
    ssao_intensity2 : f32,
    ssao_direct_light_affect : f32,
    ssao_quality : u8,
    ssao_radius : f32,
    ssao_radius2 : f32,
    tone_map_exposure : f32,
    tone_mapper : u8,
    tone_map_white : f32,
    glow_high_quality : bool,
}


impl FromWorld for WorldEnvironment {
    fn from_world(_world: &mut World) -> Self {
        WorldEnvironment {
            adjustment_brightness: 0.,
            adjustment_contrast: 0.,
            adjustment_enabled: false,
            adjustment_saturation: 0.,
            ambient_light_color: Color::GOLD,
            ambient_light_energy: 0.,
            ambient_light_sky_contribution: 0.,
            tonemap_auto_exposure: false,
            tonemap_auto_exposure_max: 0.,
            tonemap_auto_exposure_min: 0.,
            tonemap_auto_exposure_grey: 0.,
            tonemap_auto_exposure_speed: 0.,
            camera_feed_id: 0,
            canvas_max_layer: 0,
            bg_color: Color::GOLD,
            bg_energy: 0.,
            background_mode: 0,
            sky_custom_fov: 0.,
            sky_orientation: Quat::IDENTITY,
            dof_blur_far_amount: 0.,
            dof_blur_far_distance: 0.,
            dof_blur_far_enabled: false,
            dof_blur_far_quality: 0,
            dof_blur_far_transition: 0.,
            dof_blur_near_amount: 0.,
            dof_blur_near_distance: 0.,
            dof_blur_near_enabled: false,
            dof_blur_near_quality: 0.,
            dof_blur_near_transition: 0.,
            fog_color: Color::GOLD,
            fog_depth_begin: 0.,
            fog_depth_curve: 0.,
            fog_depth_enabled: false,
            fog_depth_end: 0.,
            fog_enabled: false,
            fog_height_curve: 0.,
            fog_height_enabled: false,
            fog_height_max: 0.,
            fog_height_min: 0.,
            fog_sun_amount: 0.,
            fog_sun_color: Color::GOLD,
            fog_transmit_curve: 0.,
            fog_transmit_enabled: false,
            glow_bicubic_upscale_enabled: false,
            glow_blend_mode: 0,
            glow_bloom: 0.,
            glow_enabled: false,
            glow_hdr_luminance_cap: 0.,
            glow_hdr_bleed_scale: 0.,
            glow_hdr_bleed_treshold: 0.,
            glow_intensity: 0.,
            glow_strength: 0.,
            ssr_depth_tolerance: 0.,
            ssr_enabled: false,
            ssr_fade_in: 0.,
            ssr_fade_out: 0.,
            ssr_max_steps: 0,
            ssr_rough: false,
            ssao_ao_channel_affect: 0.,
            ssao_bias: 0.,
            ssao_blur: 0,
            ssao_color: Color::GOLD,
            ssao_edge_sharpness: 0.,
            ssao_enabled: false,
            ssao_intensity: 0.,
            ssao_intensity2: 0.,
            ssao_direct_light_affect: 0.,
            ssao_quality: 0,
            ssao_radius: 0.,
            ssao_radius2: 0.,
            tone_map_exposure: 0.,
            tone_mapper: 0,
            tone_map_white: 0.,
            glow_high_quality: false,
        }
    }
}
