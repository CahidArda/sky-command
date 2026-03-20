use bevy::prelude::*;

pub mod atmosphere;
pub mod flight_model;

use crate::aircraft::{Aircraft, ControlInput, Propeller};
use flight_model::{AERO_YAW_COEFF, Q_CRUISE};

pub struct PhysicsPlugin;

/// System sets for ordering physics systems.
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
                .chain(),
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
/// When there's sideslip (β ≠ 0), the vertical tail creates a yawing
/// moment. This is what makes banked turns change heading:
///   bank → tilted lift curves velocity → β develops → aero yaw rotates nose.
/// Proportional to dynamic pressure, so it's negligible in a stall.
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

        let vel_normalized = aircraft.velocity.normalize();
        let right = transform.right().as_vec3();
        let up = transform.up().as_vec3();

        // Sideslip angle β
        let dot_right = vel_normalized.dot(right);
        let beta = dot_right.clamp(-1.0, 1.0).asin();

        // Yaw rate proportional to β and dynamic pressure
        let q_scale = q / Q_CRUISE;
        let aero_yaw_rate = beta * AERO_YAW_COEFF * q_scale;

        // Rotate around the aircraft's local up axis (negate to reduce β)
        let yaw_rot = Quat::from_axis_angle(up, -aero_yaw_rate * dt);
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
            let spin_speed = aircraft.throttle * 30.0; // radians per second
            transform.rotate_local_z(spin_speed * time.delta_secs());
        }
    }
}
