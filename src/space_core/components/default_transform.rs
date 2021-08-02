use bevy::prelude::Transform;

pub struct DefaultTransform {

    pub transform : Transform,

}

impl Default for DefaultTransform {
    fn default() -> Self {
        Self {
            transform : Transform::identity(),
        }
    }
}
