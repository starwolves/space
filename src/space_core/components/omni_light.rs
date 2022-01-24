use bevy::prelude::{Color, Component};
#[derive(Component)]
pub struct OmniLight {
    pub omni_attenuation : f32,
    pub omni_range : f32,
    pub omni_shadow_detail : u8,
    pub omni_shadow_mode : u8,
    pub bake_mode : u8,
    pub color : Color,
    pub cull_mask : i64,
    pub light_energy : f32,
    pub light_indirect_energy : f32,
    pub negative : bool,
    pub light_specular : f32,
    pub shadow_bias : f32,
    pub shadow_color : Color,
    pub shadow_contact : f32,
    pub shadow : bool,
    pub shadow_reverse_cull_face : bool
}
