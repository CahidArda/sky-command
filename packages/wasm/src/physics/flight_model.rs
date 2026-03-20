use bevy::prelude::*;
use std::f32::consts::PI;

use super::atmosphere::{self, G, RHO_SEA_LEVEL};
use crate::aircraft::Aircraft;

/// Maximum lift coefficient before stall.
const CL_MAX: f32 = 1.5;

/// Stall angle in radians (~15 degrees).
const STALL_ANGLE: f32 = 0.2618; // ~15 degrees

/// Compute coefficient of lift as a function of angle of attack.
///
/// Cl = 2*pi*alpha for small angles (thin airfoil theory).
/// After stall angle (~15 deg), lift drops off.
fn coefficient_of_lift(alpha: f32) -> f32 {
    let alpha_clamped = alpha.clamp(-0.5, 0.5);
    if alpha_clamped.abs() < STALL_ANGLE {
        // Linear region: Cl = 2*pi*alpha
        let cl = 2.0 * PI * alpha_clamped;
        cl.clamp(-CL_MAX, CL_MAX)
    } else {
        // Post-stall: lift drops off
        let sign = alpha_clamped.signum();
        let beyond_stall = (alpha_clamped.abs() - STALL_ANGLE) / (0.5 - STALL_ANGLE);
        let cl_at_stall = 2.0 * PI * STALL_ANGLE;
        let cl = cl_at_stall * (1.0 - 0.6 * beyond_stall);
        sign * cl.max(0.2)
    }
}

/// Compute the angle of attack from velocity and aircraft orientation.
///
/// Alpha is the angle between the velocity vector and the aircraft's
/// forward direction, measured in the aircraft's pitch plane.
fn compute_alpha(velocity_local: Vec3) -> f32 {
    let speed = velocity_local.length();
    if speed < 1.0 {
        return 0.0;
    }
    // In Bevy local space, forward is -Z, up is +Y.
    // For straight-and-level flight, velocity_local ≈ (0, 0, -speed).
    // alpha = atan2(-y, -z) so that forward flight gives alpha = 0.
    (-velocity_local.y).atan2(-velocity_local.z)
}

/// Main flight physics system.
///
/// Computes lift, drag, thrust, and weight forces, then integrates
/// velocity. All computation is in SI units.
pub fn update_flight_physics(
    time: Res<Time>,
    mut query: Query<(&mut Aircraft, &Transform)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 || dt > 0.1 {
        return; // Skip bad timesteps
    }

    for (mut aircraft, transform) in query.iter_mut() {
        let altitude = transform.translation.y;
        let rho = atmosphere::density(altitude);

        // Aircraft local axes from its rotation
        let forward = transform.forward().as_vec3();
        let up = transform.up().as_vec3();
        let _right = transform.right().as_vec3();

        // Transform velocity into local space for aerodynamic calculations
        let rotation = transform.rotation;
        let velocity_local = rotation.inverse() * aircraft.velocity;
        let speed = aircraft.velocity.length();

        // Dynamic pressure: q = 0.5 * rho * V^2
        let q = 0.5 * rho * speed * speed;

        // Angle of attack
        let alpha = compute_alpha(velocity_local);

        // ---- LIFT ----
        // Lift = q * S * Cl(alpha)
        // Lift acts perpendicular to velocity, in the plane of aircraft up and velocity
        let cl = coefficient_of_lift(alpha);
        let lift_magnitude = q * aircraft.wing_area * cl;

        // Lift direction: perpendicular to velocity in the aircraft's pitch plane
        let lift_dir = if speed > 1.0 {
            let vel_normalized = aircraft.velocity.normalize();
            // Lift is perpendicular to velocity, biased toward aircraft up
            let lift_raw = up - vel_normalized * up.dot(vel_normalized);
            let len = lift_raw.length();
            if len > 0.001 {
                lift_raw / len
            } else {
                Vec3::Y
            }
        } else {
            Vec3::Y
        };

        let lift_force = lift_dir * lift_magnitude;

        // ---- DRAG ----
        // Cd = Cd0 + Cl^2 / (pi * e * AR)
        let induced_drag_coeff =
            cl * cl / (PI * aircraft.oswald_efficiency * aircraft.aspect_ratio);
        let cd = aircraft.cd0 + induced_drag_coeff;
        let drag_magnitude = q * aircraft.wing_area * cd;

        // Drag acts opposite to velocity
        let drag_force = if speed > 0.1 {
            -aircraft.velocity.normalize() * drag_magnitude
        } else {
            Vec3::ZERO
        };

        // ---- THRUST ----
        // Thrust = throttle * max_thrust * (rho / rho_sea_level)
        // Thrust acts along aircraft forward axis
        let density_ratio = rho / RHO_SEA_LEVEL;
        let thrust_magnitude = aircraft.throttle * aircraft.max_thrust * density_ratio;
        let thrust_force = forward * thrust_magnitude;

        // ---- WEIGHT ----
        let weight_force = Vec3::new(0.0, -aircraft.mass * G, 0.0);

        // ---- SUM FORCES ----
        let total_force = lift_force + drag_force + thrust_force + weight_force;

        // ---- ACCELERATION AND INTEGRATION ----
        let acceleration = total_force / aircraft.mass;
        aircraft.velocity += acceleration * dt;

        // Clamp velocity to prevent numerical instability
        let max_speed = 200.0; // m/s (~389 knots)
        if aircraft.velocity.length() > max_speed {
            aircraft.velocity = aircraft.velocity.normalize() * max_speed;
        }

        // Ground interaction: if on or below ground, prevent negative Y velocity
        // and provide a ground reaction force
        if altitude <= 0.5 {
            if aircraft.velocity.y < 0.0 {
                aircraft.velocity.y = 0.0;
            }
            // Add ground friction when on the ground
            if speed > 0.1 {
                let friction = -aircraft.velocity.normalize() * 0.02 * aircraft.mass * G;
                aircraft.velocity += friction * dt;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cl_at_zero_alpha() {
        let cl = coefficient_of_lift(0.0);
        assert!(cl.abs() < 0.001);
    }

    #[test]
    fn cl_linear_region() {
        let alpha = 0.1; // ~5.7 degrees
        let cl = coefficient_of_lift(alpha);
        let expected = 2.0 * PI * alpha;
        assert!((cl - expected).abs() < 0.01);
    }

    #[test]
    fn cl_post_stall_drops() {
        let cl_at_stall = coefficient_of_lift(STALL_ANGLE);
        let cl_past_stall = coefficient_of_lift(0.4);
        assert!(cl_past_stall < cl_at_stall);
    }

    #[test]
    fn alpha_straight_and_level() {
        // Velocity purely in forward direction (local -Z in Bevy)
        let alpha = compute_alpha(Vec3::new(0.0, 0.0, -60.0));
        assert!(alpha.abs() < 0.001);
    }

    #[test]
    fn alpha_nose_up() {
        // Velocity has downward component in local space = positive alpha (nose above flight path)
        let alpha = compute_alpha(Vec3::new(0.0, -10.0, -60.0));
        assert!(alpha > 0.0);
    }
}
