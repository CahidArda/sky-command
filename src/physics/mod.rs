use bevy::prelude::*;

pub mod atmosphere;
pub mod flight_model;

use crate::aircraft::{
    Aircraft, AileronLeft, AileronRight, ControlInput, Elevator, Propeller, Rudder,
};
use crate::state::GameState;
use flight_model::{AERO_YAW_COEFF, Q_CRUISE, STALL_ANGLE};

pub struct PhysicsPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicsSet {
    Forces,
    Integration,
    TransformSync,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                PhysicsSet::Forces,
                PhysicsSet::Integration,
                PhysicsSet::TransformSync,
            )
                .chain()
                .run_if(in_state(GameState::Flying)),
        )
        .add_systems(
            Update,
            (
                update_throttle.in_set(PhysicsSet::Forces),
                update_angular_velocity.in_set(PhysicsSet::Forces),
                flight_model::update_flight_physics.in_set(PhysicsSet::Integration),
                aerodynamic_yaw.in_set(PhysicsSet::TransformSync),
                update_aircraft_transform.in_set(PhysicsSet::TransformSync).after(aerodynamic_yaw),
                spin_propeller.in_set(PhysicsSet::TransformSync),
                animate_control_surfaces.in_set(PhysicsSet::TransformSync),
            ),
        );
    }
}

/// Update throttle based on control input.
fn update_throttle(
    time: Res<Time>,
    mut query: Query<(&mut Aircraft, &ControlInput)>,
) {
    let dt = time.delta_secs();
    for (mut aircraft, input) in query.iter_mut() {
        aircraft.throttle += input.throttle_change * dt * 0.5;
        aircraft.throttle = aircraft.throttle.clamp(0.0, 1.0);
    }
}

/// Update angular velocity based on control input and apply rotation.
fn update_angular_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Aircraft, &ControlInput, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (mut aircraft, input, mut transform) in query.iter_mut() {
        // Target angular velocities based on input
        let target_pitch = input.pitch * aircraft.pitch_rate;
        let target_roll = input.roll * aircraft.roll_rate;
        let target_yaw = input.yaw * aircraft.yaw_rate;

        // Smoothly interpolate angular velocity toward target
        let lerp_rate = 5.0 * dt;
        aircraft.angular_velocity.x = aircraft.angular_velocity.x
            + (target_pitch - aircraft.angular_velocity.x) * lerp_rate.min(1.0);
        aircraft.angular_velocity.y = aircraft.angular_velocity.y
            + (target_yaw - aircraft.angular_velocity.y) * lerp_rate.min(1.0);
        aircraft.angular_velocity.z = aircraft.angular_velocity.z
            + (target_roll - aircraft.angular_velocity.z) * lerp_rate.min(1.0);

        // When no input, dampen angular velocity
        if input.pitch.abs() < 0.01 {
            aircraft.angular_velocity.x *= (1.0 - 3.0 * dt).max(0.0);
        }
        if input.yaw.abs() < 0.01 {
            aircraft.angular_velocity.y *= (1.0 - 3.0 * dt).max(0.0);
        }
        if input.roll.abs() < 0.01 {
            aircraft.angular_velocity.z *= (1.0 - 3.0 * dt).max(0.0);
        }

        // Apply rotation in local space
        let pitch_rot = Quat::from_axis_angle(Vec3::X, aircraft.angular_velocity.x * dt);
        let yaw_rot = Quat::from_axis_angle(Vec3::Y, aircraft.angular_velocity.y * dt);
        let roll_rot = Quat::from_axis_angle(Vec3::Z, aircraft.angular_velocity.z * dt);

        transform.rotation = transform.rotation * (yaw_rot * pitch_rot * roll_rot);
        transform.rotation = transform.rotation.normalize();
    }
}

/// Aerodynamic yaw: rotates the aircraft nose toward the velocity direction.
///
/// The heading error is computed in the AIRCRAFT'S yaw plane (perpendicular
/// to the aircraft's local up), and the rotation is applied around the
/// aircraft's local up axis. This works correctly at all bank angles —
/// it doesn't create parasitic pitch like world-Y rotation does.
fn aerodynamic_yaw(
    time: Res<Time>,
    mut query: Query<(&Aircraft, &mut Transform)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (aircraft, mut transform) in query.iter_mut() {
        let speed = aircraft.velocity.length();
        if speed < 1.0 {
            continue;
        }

        let altitude = transform.translation.y;
        let rho = atmosphere::density(altitude);
        let q = 0.5 * rho * speed * speed;

        let forward = transform.forward().as_vec3();
        let up = transform.up().as_vec3();
        let vel_dir = aircraft.velocity.normalize();

        // Project forward and velocity onto the aircraft's yaw plane
        // (perpendicular to the aircraft's local up).
        let fwd_proj = forward - up * forward.dot(up);
        let vel_proj = vel_dir - up * vel_dir.dot(up);

        let fwd_len = fwd_proj.length();
        let vel_len = vel_proj.length();
        if fwd_len < 0.001 || vel_len < 0.001 {
            continue;
        }

        let fwd_n = fwd_proj / fwd_len;
        let vel_n = vel_proj / vel_len;

        // Signed angle from forward to velocity around the up axis
        let cross = fwd_n.cross(vel_n).dot(up);
        let dot = fwd_n.dot(vel_n);
        let heading_error = cross.atan2(dot);

        // Bank factor: mostly active when banked, weak when level.
        let bank_factor = ((1.0 - up.y * up.y).max(0.0).sqrt()).max(0.1);

        // At high AoA (> stall), the vertical tail is also stalled and
        // loses effectiveness. Scale aero yaw down to prevent violent spin.
        let alpha_abs = aircraft.alpha.abs();
        let stall_fade = if alpha_abs > STALL_ANGLE {
            (1.0 - ((alpha_abs - STALL_ANGLE) / (1.0 - STALL_ANGLE)).min(1.0)).max(0.05)
        } else {
            1.0
        };

        let q_scale = q / Q_CRUISE;
        let yaw_rate = (heading_error * AERO_YAW_COEFF * q_scale * bank_factor * stall_fade)
            .clamp(-2.0, 2.0); // max 2 rad/s ≈ 115°/s to prevent violent spin

        // Rotate around the aircraft's LOCAL up axis
        let yaw_rot = Quat::from_axis_angle(up, yaw_rate * dt);
        transform.rotation = yaw_rot * transform.rotation;
        transform.rotation = transform.rotation.normalize();
    }
}

/// Sync the Aircraft velocity into the Transform position.
fn update_aircraft_transform(
    time: Res<Time>,
    mut query: Query<(&Aircraft, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (aircraft, mut transform) in query.iter_mut() {
        transform.translation += aircraft.velocity * dt;

        // Ground collision: prevent going below ground level
        if transform.translation.y < 0.5 {
            transform.translation.y = 0.5;
            // Zero out downward velocity if we hit the ground
            if aircraft.velocity.y < 0.0 {
                // We don't mutate aircraft here — flight_model handles it
            }
        }
    }
}

/// Spin the propeller based on throttle.
fn spin_propeller(
    time: Res<Time>,
    aircraft_query: Query<&Aircraft>,
    mut prop_query: Query<(&Parent, &mut Transform), With<Propeller>>,
) {
    for (parent, mut transform) in prop_query.iter_mut() {
        if let Ok(aircraft) = aircraft_query.get(parent.get()) {
            let spin_speed = aircraft.throttle * 30.0;
            transform.rotate_local_z(spin_speed * time.delta_secs());
        }
    }
}

/// Animate control surfaces (ailerons, elevator, rudder) based on input.
/// Max deflection ≈ 25° (0.44 rad). Surfaces lerp smoothly to target.
const MAX_SURFACE_DEFLECTION: f32 = 0.44;
const SURFACE_LERP_SPEED: f32 = 10.0;

fn animate_control_surfaces(
    time: Res<Time>,
    aircraft_query: Query<&ControlInput>,
    mut left_ail: Query<(&Parent, &mut Transform), (With<AileronLeft>, Without<AileronRight>, Without<Elevator>, Without<Rudder>)>,
    mut right_ail: Query<(&Parent, &mut Transform), (With<AileronRight>, Without<AileronLeft>, Without<Elevator>, Without<Rudder>)>,
    mut elevator: Query<(&Parent, &mut Transform), (With<Elevator>, Without<AileronLeft>, Without<AileronRight>, Without<Rudder>)>,
    mut rudder: Query<(&Parent, &mut Transform, &Rudder), (Without<AileronLeft>, Without<AileronRight>, Without<Elevator>)>,
) {
    let dt = time.delta_secs();
    let t = (SURFACE_LERP_SPEED * dt).min(1.0);

    for (parent, mut tf) in left_ail.iter_mut() {
        if let Ok(input) = aircraft_query.get(parent.get()) {
            let target = Quat::from_rotation_x(-input.roll * MAX_SURFACE_DEFLECTION);
            tf.rotation = tf.rotation.slerp(target, t);
        }
    }
    for (parent, mut tf) in right_ail.iter_mut() {
        if let Ok(input) = aircraft_query.get(parent.get()) {
            let target = Quat::from_rotation_x(input.roll * MAX_SURFACE_DEFLECTION);
            tf.rotation = tf.rotation.slerp(target, t);
        }
    }
    for (parent, mut tf) in elevator.iter_mut() {
        if let Ok(input) = aircraft_query.get(parent.get()) {
            let target = Quat::from_rotation_x(-input.pitch * MAX_SURFACE_DEFLECTION);
            tf.rotation = tf.rotation.slerp(target, t);
        }
    }
    // Rudder: compose deflection on top of base rotation (handles canted F-15 tails)
    for (parent, mut tf, rudder_data) in rudder.iter_mut() {
        if let Ok(input) = aircraft_query.get(parent.get()) {
            let deflection = Quat::from_rotation_y(-input.yaw * MAX_SURFACE_DEFLECTION);
            let target = rudder_data.base_rotation * deflection;
            tf.rotation = tf.rotation.slerp(target, t);
        }
    }
}
