use std::{collections::HashMap, f32::consts::PI};

use api::{
    data::{HandleToEntity, Showcase, TickRate, ZeroGravity},
    gridmap::world_to_cell_id,
    inventory::Inventory,
};
use bevy::{
    hierarchy::Children,
    math::{Quat, Vec2, Vec3},
    prelude::{
        warn, Commands, Component, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform,
        With, Without,
    },
    time::{Time, Timer},
};
use combat::{active_attacks::ActiveAttackIncrement, attack::Attack};
use examinable::examine::Examinable;
use gridmap::grid::GridmapMain;
use health::core::DamageFlag;
use inventory_item::{
    combat::{MeleeCombat, ProjectileCombat},
    item::{CombatStandardAnimation, InventoryItem},
};
use networking::messages::{InputToggleCombatMode, NetUnloadEntity};
use pawn::pawn::{
    facing_direction_to_direction, ControllerInput, FacingDirection, Pawn, PawnYAxisRotations,
};
use rigid_body::rigid_body::RigidBodyData;
use sensable::core::Sensable;
use sfx::builder::repeating_sfx_builder;
use sounds::actions::{
    footsteps_sprinting_sfx::FootstepsSprintingSfxBundle,
    footsteps_walking_sfx::FootstepsWalkingSfxBundle,
};

use bevy_rapier3d::{
    na::UnitQuaternion,
    prelude::{CoefficientCombineRule, Collider, Dominance, ExternalForce, Friction, Velocity},
};

use std::time::Duration;

/// Component link repeated footstep sfx with an entity.
#[derive(Component)]
pub(crate) struct LinkedFootstepsSprinting {
    pub entity: Entity,
}

/// Component link repeated footstep sfx with an entity.
#[derive(Component)]
pub(crate) struct LinkedFootstepsWalking {
    pub entity: Entity,
}

/// Humanoid character animation state.
pub enum CharacterAnimationState {
    Idle,
    Jogging,
    Sprinting,
}

const FIRST_MELEE_TIME: u64 = 433;

/// The humanoid component.
#[derive(Component)]
pub struct Humanoid {
    /// Lower body blended animation state of humanoid.
    pub current_lower_animation_state: CharacterAnimationState,
    /// In-game name.
    pub character_name: String,
    /// Whether the humanoid is in combat mode or normal mode.
    pub combat_mode: bool,
    /// The the humanoid is facing.
    pub facing_direction: f32,
    /// If attacking this frame.
    pub is_attacking: bool,
    /// Timeout between attacks.
    pub next_attack_timer: Timer,
}

impl Default for Humanoid {
    fn default() -> Self {
        let mut t = Timer::new(Duration::from_millis(FIRST_MELEE_TIME), false);
        let mut first_damage_flags = HashMap::new();
        first_damage_flags.insert(0, DamageFlag::SoftDamage);
        t.tick(Duration::from_millis(FIRST_MELEE_TIME));
        Self {
            current_lower_animation_state: CharacterAnimationState::Idle,
            character_name: "".to_string(),
            combat_mode: false,
            facing_direction: 0.,
            is_attacking: false,
            next_attack_timer: t,
        }
    }
}

/// Toggle combat mode. Ie from melee to projectile.
pub(crate) fn toggle_combat_mode(
    mut toggle_combat_mode_events: EventReader<InputToggleCombatMode>,
    mut standard_character_query: Query<&mut Humanoid>,
) {
    for event in toggle_combat_mode_events.iter() {
        match standard_character_query.get_mut(event.entity) {
            Ok(mut standard_character) => {
                standard_character.combat_mode = !standard_character.combat_mode;
            }
            Err(_rr) => {}
        }
    }
}

const JOG_SPEED: f32 = 3031.44;
const RUN_SPEED: f32 = 3031.44;

const MAX_JOG_SPEED: f32 = 10.;
const MAX_RUN_SPEED: f32 = 14.;

const COMBAT_ROTATION_SPEED: f32 = 18.;
const DOWN_FORCE: f32 = -1.0;

/// Animation movement state.
enum CharacterMovementState {
    None,
    Jogging,
    Sprinting,
}

/// Core humanoid logic.
pub(crate) fn humanoid_core(
    mut humanoids_query: Query<
        (
            Entity,
            &mut ControllerInput,
            &Velocity,
            &mut Dominance,
            &mut ExternalForce,
            &mut Humanoid,
            Option<&LinkedFootstepsWalking>,
            Option<&LinkedFootstepsSprinting>,
            &mut Pawn,
            &Inventory,
            Option<&ZeroGravity>,
            &RigidBodyData,
            &Children,
        ),
        Without<Showcase>,
    >,
    mut transforms: Query<&mut Transform>,
    mut colliders: Query<&mut Friction, With<Collider>>,
    inventory_items_query: Query<(
        &InventoryItem,
        &Examinable,
        &MeleeCombat,
        Option<&ProjectileCombat>,
    )>,
    mut sensable_entities: Query<&mut Sensable>,
    time: Res<Time>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands: Commands,
    tick_rate: Res<TickRate>,
    mut attack_event_writer: EventWriter<Attack>,
    tuple0: (EventWriter<NetUnloadEntity>,),
    gridmap_main: Res<GridmapMain>,
    mut attack_events: ResMut<ActiveAttackIncrement>,
) {
    let (mut net_unload_entity,) = tuple0;

    for (
        standard_character_entity,
        mut player_input_component,
        rigid_body_velocity_component,
        mut rigid_body_dominance,
        mut rigid_body_forces,
        mut humanoid_component,
        linked_footsteps_walking_option,
        linked_footsteps_sprinting_option,
        mut pawn_component,
        inventory_component,
        zero_gravity_component_option,
        rigidbody_data_component,
        children,
    ) in humanoids_query.iter_mut()
    {
        let mut collider_child_entity_option = None;

        for child in children.iter() {
            match colliders.get(*child) {
                Ok(_f) => {
                    collider_child_entity_option = Some(*child);
                }
                Err(_rr) => {}
            }
        }

        let mut collider_material_component;

        match collider_child_entity_option {
            Some(c) => {
                collider_material_component = colliders.get_mut(c).unwrap();
            }
            None => {
                warn!("Couldnt find collider child of pawn.");
                continue;
            }
        }

        let movement_options = PawnYAxisRotations::new();

        let character_movement_state;

        if player_input_component.auto_move_enabled {
            if player_input_component.movement_vector.length() > 0.1 {
                player_input_component.auto_move_direction =
                    player_input_component.movement_vector.clone();
            }
        } else {
            player_input_component.auto_move_direction = Vec2::ZERO;
        }

        if humanoid_component.combat_mode == false {
            if player_input_component.is_mouse_action_pressed {
                player_input_component.is_mouse_action_pressed = false;
            }
        }

        let mut speed_factor = JOG_SPEED;

        if player_input_component.sprinting {
            speed_factor = RUN_SPEED;
        }

        let player_input_movement_vector;

        if player_input_component.auto_move_enabled
            && player_input_component.movement_vector.length() < 0.1
        {
            if player_input_component.auto_move_direction.length() < 0.1 {
                player_input_movement_vector =
                    facing_direction_to_direction(&pawn_component.facing_direction);
                player_input_component.auto_move_direction = player_input_movement_vector;
            } else {
                player_input_movement_vector = player_input_component.auto_move_direction;
            }
        } else {
            player_input_movement_vector = player_input_component.movement_vector;
        }

        if player_input_movement_vector.x.abs() == 1. && player_input_movement_vector.y.abs() == 1.
        {
            speed_factor *= 0.75;
        }

        if player_input_movement_vector.length() == 0. {
            character_movement_state = CharacterMovementState::None;
        } else {
            if player_input_component.sprinting {
                character_movement_state = CharacterMovementState::Sprinting;
            } else {
                character_movement_state = CharacterMovementState::Jogging;
            }
        }

        let delta_time = time.delta();
        let delta_seconds = delta_time.as_secs_f32();

        let mut netto_force = Vec3::new(
            player_input_movement_vector.x * -speed_factor,
            DOWN_FORCE,
            player_input_movement_vector.y * speed_factor,
        );

        let bevy_vec: Vec3 = rigid_body_forces.force.into();
        netto_force += bevy_vec;

        let mut rigid_body_position;
        let character_pos;

        match transforms.get(standard_character_entity) {
            Ok(rigid_body_position_component) => {
                rigid_body_position = rigid_body_position_component.clone();
                character_pos = rigid_body_position.clone();
            }
            Err(_) => {
                warn!("Couldnt find pawn transform!");
                continue;
            }
        }

        let mut movement_index: usize = 0;

        let mut idle = false;

        let mut facing_direction = pawn_component.facing_direction.clone();

        humanoid_component.next_attack_timer.tick(delta_time);
        let ready_to_attack_this_frame = humanoid_component.next_attack_timer.finished();

        // If combat mode, specific new rotation based on mouse direction.
        if humanoid_component.combat_mode && !player_input_component.sprinting {
            let active_slot = inventory_component.get_slot(&inventory_component.active_slot);

            let mut rotation_offset = -0.1 * PI;

            if &inventory_component.active_slot == "right_hand" {
                rotation_offset = 0.11 * PI;
            }

            let mut alt_attack_mode = false;

            let projectile_combat_component_option;

            match active_slot.slot_item {
                Some(item_entity) => match inventory_items_query.get(item_entity) {
                    Ok((
                        item_component,
                        _examinable_component,
                        _melee_combat,
                        projectile_combat_component,
                    )) => {
                        projectile_combat_component_option = projectile_combat_component;

                        match item_component.combat_standard_animation {
                            CombatStandardAnimation::StandardStance => {}
                            CombatStandardAnimation::PistolStance => {
                                alt_attack_mode = player_input_component.alt_attack_mode
                                    && projectile_combat_component_option.is_some();

                                if !alt_attack_mode {
                                    if player_input_movement_vector.x != 0.
                                        || player_input_movement_vector.y != 0.
                                    {
                                        rotation_offset = -0.0675 * PI;
                                    } else {
                                        rotation_offset = -0.24 * PI;
                                    }
                                }
                            }
                        }
                    }
                    Err(_rr) => {
                        warn!("Couldn't find inventory_item belonging to used inventory slot of attack.");
                    }
                },
                None => {}
            }

            let end_rotation = Quat::from_axis_angle(
                Vec3::new(0., 1., 0.),
                -humanoid_component.facing_direction - 0.5 * PI + rotation_offset,
            );

            let mut rigid_body_transform = character_pos;

            let slerp_rotation;

            if rigid_body_transform.rotation.dot(end_rotation) > 0. {
                slerp_rotation = rigid_body_transform
                    .rotation
                    .slerp(end_rotation, delta_seconds * COMBAT_ROTATION_SPEED);
            } else {
                let start_rotation = -rigid_body_transform.rotation.clone();
                slerp_rotation =
                    start_rotation.slerp(end_rotation, delta_seconds * COMBAT_ROTATION_SPEED);
            }

            rigid_body_transform.rotation = slerp_rotation;

            match transforms.get_mut(standard_character_entity) {
                Ok(mut rigid_body_position_component) => {
                    rigid_body_position_component.translation = rigid_body_transform.translation;
                    rigid_body_position_component.rotation = rigid_body_transform.rotation;
                    rigid_body_position_component.scale = rigid_body_transform.scale;
                }
                Err(_) => {
                    warn!("Couldnt find pawn transform!1");
                    continue;
                }
            }

            let mut attacking_this_frame = false;

            if player_input_component.is_mouse_action_pressed {
                if ready_to_attack_this_frame {
                    attacking_this_frame = true;
                }
                if ready_to_attack_this_frame {
                    humanoid_component.next_attack_timer.reset()
                }
                if humanoid_component.next_attack_timer.paused() {
                    humanoid_component.next_attack_timer.unpause();
                    humanoid_component.next_attack_timer.reset();
                }
                if !humanoid_component.is_attacking {
                    humanoid_component.is_attacking = true;
                }
            } else {
                if humanoid_component.is_attacking {
                    humanoid_component.is_attacking = false;
                }
            }

            if attacking_this_frame {
                // Get used inventory item and attack mode enum. Then on match execute directPreciseRayCastMeleeAttack

                let mut angle = humanoid_component.facing_direction;

                if angle < 0. {
                    angle = -PI - angle;
                } else {
                    angle = PI - angle;
                }

                let mut exclude_entity = vec![];
                match active_slot.slot_item {
                    Some(e) => {
                        exclude_entity.push(e);
                    }
                    None => {}
                }

                attack_event_writer.send(Attack {
                    attacker: standard_character_entity,
                    weapon_option: active_slot.slot_item,
                    incremented_id: attack_events.get_id_inc(),
                    targetted_entity: player_input_component.combat_targetted_entity.clone(),
                    targetted_cell: player_input_component.combat_targetted_cell.clone(),
                    angle,
                    targetted_limb: player_input_component.targetted_limb.clone(),
                    alt_attack_mode,
                });
            }
        }

        match &player_input_component.pending_direction {
            Some(dir) => {
                facing_direction = dir.clone();
                match facing_direction {
                    FacingDirection::UpLeft => {
                        movement_index = 1;
                    }
                    FacingDirection::Up => {
                        movement_index = 0;
                    }
                    FacingDirection::UpRight => {
                        movement_index = 7;
                    }
                    FacingDirection::Right => {
                        movement_index = 6;
                    }
                    FacingDirection::DownRight => {
                        movement_index = 3;
                    }
                    FacingDirection::Down => {
                        movement_index = 4;
                    }
                    FacingDirection::DownLeft => {
                        movement_index = 5;
                    }
                    FacingDirection::Left => {
                        movement_index = 2;
                    }
                }
            }
            None => {
                // Moving up.
                if player_input_movement_vector.y == 1. && player_input_movement_vector.x == 0. {
                    movement_index = 0;
                    facing_direction = FacingDirection::Up;
                }
                // Moving down.
                else if player_input_movement_vector.y == -1.
                    && player_input_movement_vector.x == 0.
                {
                    movement_index = 4;
                    facing_direction = FacingDirection::Down;
                }
                // Moving left.
                else if player_input_movement_vector.y == 0.
                    && player_input_movement_vector.x == -1.
                {
                    movement_index = 2;
                    facing_direction = FacingDirection::Left;
                }
                // Moving right.
                else if player_input_movement_vector.y == 0.
                    && player_input_movement_vector.x == 1.
                {
                    movement_index = 6;
                    facing_direction = FacingDirection::Right;
                }
                // Moving up left.
                else if player_input_movement_vector.y == 1.
                    && player_input_movement_vector.x == -1.
                {
                    movement_index = 1;
                    facing_direction = FacingDirection::UpLeft;
                }
                // Moving up right.
                else if player_input_movement_vector.y == 1.
                    && player_input_movement_vector.x == 1.
                {
                    movement_index = 7;
                    facing_direction = FacingDirection::UpRight;
                }
                // Moving down left.
                else if player_input_movement_vector.y == -1.
                    && player_input_movement_vector.x == -1.
                {
                    movement_index = 3;
                    facing_direction = FacingDirection::DownRight;
                }
                // Moving down right.
                else if player_input_movement_vector.y == -1.
                    && player_input_movement_vector.x == 1.
                {
                    movement_index = 5;
                    facing_direction = FacingDirection::DownLeft;
                } else if player_input_movement_vector.y == 0.
                    && player_input_movement_vector.x == 0.
                {
                    idle = true;
                }
            }
        }

        player_input_component.pending_direction = None;

        pawn_component.facing_direction = facing_direction;

        match (humanoid_component.combat_mode && idle)
            || (humanoid_component.combat_mode == false && idle)
        {
            true => {
                if matches!(
                    humanoid_component.current_lower_animation_state,
                    CharacterAnimationState::Jogging
                ) {
                    humanoid_component.current_lower_animation_state =
                        CharacterAnimationState::Idle;
                    // Despawn FootstepsWalkingSfx here.

                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {
                            let mut sensable_component = sensable_entities
                                .get_mut(linked_footsteps_walking_component.entity)
                                .unwrap();

                            sensable_component.despawn(
                                linked_footsteps_walking_component.entity,
                                &mut net_unload_entity,
                                &handle_to_entity,
                            );

                            commands
                                .entity(standard_character_entity)
                                .remove::<LinkedFootstepsWalking>();

                            commands
                                .entity(linked_footsteps_walking_component.entity)
                                .despawn();
                        }
                        None => {}
                    }
                }

                if matches!(
                    humanoid_component.current_lower_animation_state,
                    CharacterAnimationState::Sprinting
                ) {
                    humanoid_component.current_lower_animation_state =
                        CharacterAnimationState::Idle;
                    // Despawn FootstepsSprintingSfx here.

                    match linked_footsteps_sprinting_option {
                        Some(linked_footsteps_sprinting_component) => {
                            let mut sensable_component = sensable_entities
                                .get_mut(linked_footsteps_sprinting_component.entity)
                                .unwrap();

                            sensable_component.despawn(
                                linked_footsteps_sprinting_component.entity,
                                &mut net_unload_entity,
                                &handle_to_entity,
                            );

                            commands
                                .entity(standard_character_entity)
                                .remove::<LinkedFootstepsSprinting>();

                            commands
                                .entity(linked_footsteps_sprinting_component.entity)
                                .despawn();
                        }
                        None => {}
                    }
                }
            }
            false => {
                if humanoid_component.combat_mode == false || player_input_component.sprinting {
                    rigid_body_position.rotation =
                        UnitQuaternion::from_quaternion(movement_options[movement_index]).into();

                    match transforms.get_mut(standard_character_entity) {
                        Ok(mut rigid_body_position_component) => {
                            rigid_body_position_component.translation =
                                rigid_body_position.translation;
                            rigid_body_position_component.rotation = rigid_body_position.rotation;
                            rigid_body_position_component.scale = rigid_body_position.scale;
                        }
                        Err(_) => {
                            warn!("Couldnt find pawn transform!3");
                            continue;
                        }
                    }
                }

                if !player_input_component.sprinting
                    && matches!(
                        humanoid_component.current_lower_animation_state,
                        CharacterAnimationState::Jogging
                    ) == false
                {
                    if matches!(
                        humanoid_component.current_lower_animation_state,
                        CharacterAnimationState::Sprinting
                    ) {
                        match linked_footsteps_sprinting_option {
                            Some(linked_footsteps_sprinting_component) => {
                                let mut sensable_component = sensable_entities
                                    .get_mut(linked_footsteps_sprinting_component.entity)
                                    .unwrap();

                                sensable_component.despawn(
                                    linked_footsteps_sprinting_component.entity,
                                    &mut net_unload_entity,
                                    &handle_to_entity,
                                );

                                commands
                                    .entity(standard_character_entity)
                                    .remove::<LinkedFootstepsSprinting>();

                                commands
                                    .entity(linked_footsteps_sprinting_component.entity)
                                    .despawn();
                            }
                            None => {}
                        }
                    }

                    humanoid_component.current_lower_animation_state =
                        CharacterAnimationState::Jogging;

                    // Spawn FootstepsWalkingSfx entity here.

                    if zero_gravity_component_option.is_none() {
                        let repeating_sfx_id = repeating_sfx_builder(
                            &mut commands,
                            rigid_body_position,
                            Box::new(FootstepsWalkingSfxBundle::new),
                        );

                        commands
                            .entity(standard_character_entity)
                            .insert(LinkedFootstepsWalking {
                                entity: repeating_sfx_id,
                            });
                    }
                } else if !player_input_component.sprinting
                    && matches!(
                        humanoid_component.current_lower_animation_state,
                        CharacterAnimationState::Jogging
                    )
                {
                    // Update transform of our FootstepsWalkingSfx Entity here. (Should be moved to its own 2tick/s system eventually)

                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {
                            let linked_footsteps_walking =
                                transforms.get_mut(linked_footsteps_walking_component.entity);
                            match linked_footsteps_walking {
                                Ok(mut static_transform_component) => {
                                    static_transform_component.translation =
                                        rigid_body_position.translation;
                                    static_transform_component.rotation =
                                        rigid_body_position.rotation;
                                    static_transform_component.scale = rigid_body_position.scale;
                                }
                                Err(err) => {
                                    warn!("linked_footsteps_walking err: {}", err);
                                }
                            }
                        }
                        None => {}
                    }
                } else if player_input_component.sprinting
                    && matches!(
                        humanoid_component.current_lower_animation_state,
                        CharacterAnimationState::Sprinting
                    ) == false
                {
                    if matches!(
                        humanoid_component.current_lower_animation_state,
                        CharacterAnimationState::Jogging
                    ) {
                        match linked_footsteps_walking_option {
                            Some(linked_footsteps_walking_component) => {
                                let mut sensable_component = sensable_entities
                                    .get_mut(linked_footsteps_walking_component.entity)
                                    .unwrap();

                                sensable_component.despawn(
                                    linked_footsteps_walking_component.entity,
                                    &mut net_unload_entity,
                                    &handle_to_entity,
                                );

                                commands
                                    .entity(standard_character_entity)
                                    .remove::<LinkedFootstepsWalking>();

                                commands
                                    .entity(linked_footsteps_walking_component.entity)
                                    .despawn();
                            }
                            None => {}
                        }
                    }

                    humanoid_component.current_lower_animation_state =
                        CharacterAnimationState::Sprinting;

                    // Spawn FootstepsWalkingSfx entity here.

                    if zero_gravity_component_option.is_none() {
                        let repeating_sfx_id = repeating_sfx_builder(
                            &mut commands,
                            rigid_body_position,
                            Box::new(FootstepsSprintingSfxBundle::new),
                        );

                        commands.entity(standard_character_entity).insert(
                            LinkedFootstepsSprinting {
                                entity: repeating_sfx_id,
                            },
                        );
                    }
                } else if player_input_component.sprinting
                    && matches!(
                        humanoid_component.current_lower_animation_state,
                        CharacterAnimationState::Sprinting
                    )
                {
                    // Update transform of our FootstepsSprintingSfx Entity here. (Should be moved to its own 2tick/s system eventually)

                    match linked_footsteps_sprinting_option {
                        Some(linked_footsteps_sprinting_component) => {
                            let linked_footsteps_sprinting =
                                transforms.get_mut(linked_footsteps_sprinting_component.entity);
                            match linked_footsteps_sprinting {
                                Ok(mut static_transform_component) => {
                                    static_transform_component.translation =
                                        rigid_body_position.translation;
                                    static_transform_component.rotation =
                                        rigid_body_position.rotation;
                                    static_transform_component.scale = rigid_body_position.scale;
                                }
                                Err(err) => {
                                    warn!("linked_footsteps_sprinting err: {}", err);
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
        }

        let bevy_velocity: Vec3 = rigid_body_velocity_component.linvel.into();
        let speed = bevy_velocity.length();

        if !matches!(character_movement_state, CharacterMovementState::None) {
            let max_speed;

            match character_movement_state {
                CharacterMovementState::None => {
                    max_speed = 0.;
                }
                CharacterMovementState::Jogging => {
                    max_speed = MAX_JOG_SPEED;
                }
                CharacterMovementState::Sprinting => {
                    max_speed = MAX_RUN_SPEED;
                }
            }

            let mut should_apply;

            if speed > max_speed {
                should_apply = false;
            } else {
                should_apply = true;
            }

            let rigid_body_position_component;

            match transforms.get(standard_character_entity) {
                Ok(t) => {
                    rigid_body_position_component = t.clone();
                }
                Err(_) => {
                    warn!("Couldnt find pawn transform!4");
                    continue;
                }
            }

            if zero_gravity_component_option.is_some() {
                let mut cell_id =
                    world_to_cell_id(rigid_body_position_component.translation.into());

                cell_id.y = 0;

                let mut bordering_wall = false;

                for j in 0..8 {
                    let mut adjacent_cell_id = cell_id.clone();

                    if j == 0 {
                        adjacent_cell_id.x += 1;
                    } else if j == 1 {
                        adjacent_cell_id.x -= 1;
                    } else if j == 2 {
                        adjacent_cell_id.z += 1;
                    } else if j == 3 {
                        adjacent_cell_id.z -= 1;
                    } else if j == 4 {
                        adjacent_cell_id.x += 1;
                        adjacent_cell_id.z += 1;
                    } else if j == 5 {
                        adjacent_cell_id.x -= 1;
                        adjacent_cell_id.z -= 1;
                    } else if j == 6 {
                        adjacent_cell_id.x += 1;
                        adjacent_cell_id.z -= 1;
                    } else {
                        adjacent_cell_id.x -= 1;
                        adjacent_cell_id.z += 1;
                    }

                    match gridmap_main.grid_data.get(&adjacent_cell_id) {
                        Some(_) => {
                            bordering_wall = true;
                            break;
                        }
                        None => {}
                    }
                }
                if !bordering_wall {
                    should_apply = false;
                    if collider_material_component.coefficient != 0. {
                        collider_material_component.coefficient = 0.;
                        collider_material_component.combine_rule = CoefficientCombineRule::Min;
                    }
                } else {
                    if rigidbody_data_component.friction != collider_material_component.coefficient
                    {
                        collider_material_component.coefficient = rigidbody_data_component.friction;
                        collider_material_component.combine_rule =
                            rigidbody_data_component.friction_combine_rule;
                    }
                }
            }

            if should_apply {
                let rapier_vector = netto_force.into();
                if rigid_body_forces.force != rapier_vector {
                    rigid_body_forces.force = rapier_vector * (1. / tick_rate.physics_rate as f32);
                }
            }
        }

        // Change physics dominance based on if moving or not moving.
        if speed > 0.1 {
            if rigid_body_dominance.groups != 9 {
                rigid_body_dominance.groups = 9;
            }
        } else {
            if rigid_body_dominance.groups != 10 {
                rigid_body_dominance.groups = 10;
            }
        }

        if zero_gravity_component_option.is_some() {
            match linked_footsteps_walking_option {
                Some(linked_footsteps_walking_component) => {
                    let mut sensable_component = sensable_entities
                        .get_mut(linked_footsteps_walking_component.entity)
                        .unwrap();

                    sensable_component.despawn(
                        linked_footsteps_walking_component.entity,
                        &mut net_unload_entity,
                        &handle_to_entity,
                    );

                    commands
                        .entity(standard_character_entity)
                        .remove::<LinkedFootstepsWalking>();

                    commands
                        .entity(linked_footsteps_walking_component.entity)
                        .despawn();
                }
                None => {}
            }
            match linked_footsteps_sprinting_option {
                Some(linked_footsteps_sprinting_component) => {
                    let mut sensable_component = sensable_entities
                        .get_mut(linked_footsteps_sprinting_component.entity)
                        .unwrap();

                    sensable_component.despawn(
                        linked_footsteps_sprinting_component.entity,
                        &mut net_unload_entity,
                        &handle_to_entity,
                    );

                    commands
                        .entity(standard_character_entity)
                        .remove::<LinkedFootstepsSprinting>();

                    commands
                        .entity(linked_footsteps_sprinting_component.entity)
                        .despawn();
                }
                None => {}
            }
        }
    }
}
