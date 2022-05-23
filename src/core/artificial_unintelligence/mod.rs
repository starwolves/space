use bevy_app::{App, Plugin};

/*use self::{
    resources::ContextMapVectors,
    systems::{find_path::find_path, steer::steer},
};*/

pub mod components;
pub mod functions;
pub mod resources;
//pub mod systems;

pub struct ArtificialUnintelligencePlugin;

impl Plugin for ArtificialUnintelligencePlugin {
    fn build(&self, _app: &mut App) {
       // app.init_resource::<ContextMapVectors>()
         //   .add_system(find_path)
        //    .add_system(steer);
    }
}
