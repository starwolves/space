use bevy::prelude::{FromWorld, World};
use bevy_rapier3d::na::Quaternion;

pub struct PlayerYAxisRotations {
    pub rotations : Vec<Quaternion<f32>>
}


impl FromWorld for PlayerYAxisRotations {
    fn from_world(_world: &mut World) -> Self {
        PlayerYAxisRotations {
            rotations: vec![
                //0deg
                Quaternion::new(1.,0.,0.,0.),
                //45deg
                Quaternion::new(0.9238795, 0. , 0.3826834, 0.),
                //90deg
                Quaternion::new(0.7071068, 0., 0.7071068, 0.),
                //135deg
                Quaternion::new(0.3826834 ,0. , 0.9238795, 0.),
                //180deg
                Quaternion::new(0. ,0., 1., 0.),
                //225deg
                Quaternion::new(-0.3826834, 0., 0.9238795, 0.),
                //270deg
                Quaternion::new(-0.7071068, 0., 0.7071068, 0.),
                //315deg
                Quaternion::new(-0.9238795, 0., 0.3826834,0.),
            ]
        }
    }
}
