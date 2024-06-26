use std::collections::BTreeMap;

use bevy::{
    app::{App, Plugin, PreUpdate as BevyPreUpdate},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Query, Res, ResMut, Resource},
    },
    math::Vec3,
    render::camera::Camera,
    transform::components::Transform,
};
use networking::stamp::TickRateStamp;
use resources::{
    correction::MAX_CACHE_TICKS_AMNT,
    input::InputSet,
    modes::is_server_mode,
    ordering::{Fin, Update},
    pawn::ClientPawn,
    physics::PhysicsSet,
};

use crate::controllers::fps::control_system;

pub struct LookTransformPlugin;

impl Plugin for LookTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(BevyPreUpdate, look_transform_system.after(control_system))
            .init_resource::<LookTransformCache>()
            .add_systems(
                Update,
                (cache_client_pawn_look_transform.in_set(PhysicsSet::Cache),),
            );
        if !is_server_mode(app) {
            app.add_systems(
                Update,
                (apply_look_cache_to_peers.in_set(InputSet::ApplyLiveCache),),
            )
            .add_systems(Fin, clean_look_cache);
        }
    }
}

#[derive(Bundle)]
pub struct LookTransformBundle {
    pub transform: LookTransform,
    pub smoother: Smoother,
}
#[derive(Resource, Default, Clone)]
pub struct LookTransformCache {
    pub cache: BTreeMap<Entity, BTreeMap<u32, LookTransform>>,
}

pub(crate) fn clean_look_cache(mut cache: ResMut<LookTransformCache>) {
    // Clean cache.
    for (_, cache) in cache.cache.iter_mut() {
        if cache.len() > MAX_CACHE_TICKS_AMNT as usize {
            let mut j = 0;
            let mut is = vec![];

            for i in cache.keys().rev() {
                if j >= MAX_CACHE_TICKS_AMNT {
                    is.push(*i);
                }
                j += 1;
            }
            for i in is {
                cache.remove(&i);
            }
        }
    }
}

pub(crate) fn apply_look_cache_to_peers(
    cache: Res<LookTransformCache>,
    mut query: Query<(Entity, &mut LookTransform), (Without<ClientPawn>, Without<Camera>)>,
    stamp: Res<TickRateStamp>,
) {
    for (entity, mut look_transform) in query.iter_mut() {
        match cache.cache.get(&entity) {
            Some(look_cache) => {
                for i in look_cache.keys().rev() {
                    if i > &stamp.tick {
                        continue;
                    }
                    let lk = look_cache.get(&i).unwrap();

                    if look_transform.target != lk.target {
                        /*info!(
                            "apply_look_cache_to_peers tick {} entity {:?} target {:?}",
                            stamp.tick, entity, lk.target
                        );*/
                    }

                    *look_transform = *lk;

                    break;
                }
            }
            None => {}
        }
    }
}

pub(crate) fn cache_client_pawn_look_transform(
    query: Query<(Entity, &LookTransform), With<ClientPawn>>,
    stamp: Res<TickRateStamp>,
    mut cache: ResMut<LookTransformCache>,
) {
    for (entity, controller) in query.iter() {
        match cache.cache.get_mut(&entity) {
            Some(map) => {
                map.insert(stamp.tick, controller.clone());
            }
            None => {
                let mut map = BTreeMap::new();
                map.insert(stamp.tick, controller.clone());
                cache.cache.insert(entity, map);
            }
        }
    }
}

/// An eye and the target it's looking at. As a component, this can be modified in place of bevy's `Transform`, and the two will
/// stay in sync.
#[derive(Clone, Component, Copy, Debug, Default)]
pub struct LookTransform {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

impl From<LookTransform> for Transform {
    fn from(t: LookTransform) -> Self {
        eye_look_at_target_transform(t.eye, t.target, t.up)
    }
}

impl LookTransform {
    pub fn new(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        Self { eye, target, up }
    }

    pub fn radius(&self) -> f32 {
        (self.target - self.eye).length()
    }

    pub fn look_direction(&self) -> Option<Vec3> {
        (self.target - self.eye).try_normalize()
    }
}

fn eye_look_at_target_transform(eye: Vec3, target: Vec3, up: Vec3) -> Transform {
    // If eye and target are very close, we avoid imprecision issues by keeping the look vector a unit vector.
    let look_vector = (target - eye).normalize();
    let look_at = eye + look_vector;

    Transform::from_translation(eye).looking_at(look_at, up)
}

/// Preforms exponential smoothing on a `LookTransform`. Set the `lag_weight` between `0.0` and `1.0`, where higher is smoother.
#[derive(Component)]
pub struct Smoother {
    lag_weight: f32,
    lerp_tfm: Option<LookTransform>,
    enabled: bool,
}

impl Smoother {
    pub fn new(lag_weight: f32) -> Self {
        Self {
            lag_weight,
            lerp_tfm: None,
            enabled: true,
        }
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if self.enabled {
            // To prevent camera jumping from last lerp before disabling to the current position,
            // reset smoother state
            self.reset();
        }
    }

    pub fn set_lag_weight(&mut self, lag_weight: f32) {
        self.lag_weight = lag_weight;
    }

    pub fn smooth_transform(&mut self, new_tfm: &LookTransform) -> LookTransform {
        debug_assert!(0.0 <= self.lag_weight);
        debug_assert!(self.lag_weight < 1.0);

        let old_lerp_tfm = self.lerp_tfm.unwrap_or(*new_tfm);

        let lead_weight = 1.0 - self.lag_weight;
        let lerp_tfm = LookTransform {
            eye: old_lerp_tfm.eye * self.lag_weight + new_tfm.eye * lead_weight,
            target: old_lerp_tfm.target * self.lag_weight + new_tfm.target * lead_weight,
            up: new_tfm.up,
        };

        self.lerp_tfm = Some(lerp_tfm);

        lerp_tfm
    }

    pub fn reset(&mut self) {
        self.lerp_tfm = None;
    }
}

fn look_transform_system(
    mut cameras: Query<(&LookTransform, &mut Transform, Option<&mut Smoother>)>,
) {
    for (look_transform, mut scene_transform, smoother) in cameras.iter_mut() {
        match smoother {
            Some(mut s) if s.enabled => {
                *scene_transform = s.smooth_transform(look_transform).into()
            }
            _ => (),
        };
    }
}
