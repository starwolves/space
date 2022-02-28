use std::collections::HashMap;

use bevy_core::Timer;
use bevy_internal::prelude::{Entity, FromWorld, World};
use bevy_rapier3d::na::Quaternion;

pub struct AuthidI {
    pub i: u16,
}

impl FromWorld for AuthidI {
    fn from_world(_world: &mut World) -> Self {
        AuthidI { i: 0 }
    }
}

pub struct HandleToEntity {
    pub map: HashMap<u32, Entity>,
    pub inv_map: HashMap<Entity, u32>,
}

impl FromWorld for HandleToEntity {
    fn from_world(_world: &mut World) -> Self {
        HandleToEntity {
            map: HashMap::new(),
            inv_map: HashMap::new(),
        }
    }
}

pub struct UsedNames {
    pub names: HashMap<String, Entity>,
    pub user_names: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}

impl FromWorld for UsedNames {
    fn from_world(_world: &mut World) -> Self {
        UsedNames {
            names: HashMap::new(),
            user_names: HashMap::new(),
            player_i: 0,
            dummy_i: 0,
        }
    }
}

pub struct PlayerYAxisRotations;

impl PlayerYAxisRotations {
    pub fn new() -> Vec<Quaternion<f32>> {
        vec![
            //0deg
            Quaternion::new(1., 0., 0., 0.),
            //45deg
            Quaternion::new(0.9238795, 0., 0.3826834, 0.),
            //90deg
            Quaternion::new(0.7071068, 0., 0.7071068, 0.),
            //135deg
            Quaternion::new(0.3826834, 0., 0.9238795, 0.),
            //180deg
            Quaternion::new(0., 0., 1., 0.),
            //225deg
            Quaternion::new(-0.3826834, 0., 0.9238795, 0.),
            //270deg
            Quaternion::new(-0.7071068, 0., 0.7071068, 0.),
            //315deg
            Quaternion::new(-0.9238795, 0., 0.3826834, 0.),
        ]
    }
}

// Logic works witha timer, better as resource.
pub struct AsanaBoardingAnnouncements {
    pub announcements: Vec<(String, Timer)>,
}

impl FromWorld for AsanaBoardingAnnouncements {
    fn from_world(_world: &mut World) -> Self {
        AsanaBoardingAnnouncements {
            announcements: vec![],
        }
    }
}
