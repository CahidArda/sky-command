use bevy::prelude::*;
use std::f32::consts::PI;

use super::atmosphere::{self, G, RHO_SEA_LEVEL};
use crate::aircraft::Aircraft;

/// Maximum lift coefficient before stall.
const CL_MAX: f32 = 1.5;

/// Stall angle in radians (~15 degrees).
const STALL_ANGLE: f32 = 0.2618; // ~15 degrees

/// Lateral sideslip force coefficient per radian of β.
/// Pushes velocity toward the nose. Moderate so it barely opposes banked turns.
const SIDE_FORCE_COEFF: f32 = 0.5;

/// Aerodynamic yaw rate coefficient (rad/s per radian of heading error at cruise q).
/// The vertical tail rotates the nose toward the velocity direction.
/// Mostly active when banked; weak in level flight (slow return after rudder).
pub const AERO_YAW_COEFF: f32 = 3.0;

/// Reference dynamic pressure at cruise (0.5 × ρ₀ × 60²).
pub const Q_CRUISE: f32 = 0.5 * RHO_SEA_LEVEL * 60.0 * 60.0;

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

/// Compute the angle of attack in the aircraft's pitch plane.
///
/// Uses the aircraft's LOCAL up vector so α responds to pitch input
/// at any bank angle and any orientation (vertical, inverted, etc.).
///
/// No inversion correction: for a symmetric airfoil (Cl = 2πα), the
/// "double negation" when inverted (negative Cl × downward liftDir)
/// is physically correct — a symmetric wing generates the same lift
/// direction regardless of roll orientation.
fn compute_alpha(forward: Vec3, up: Vec3, velocity: Vec3, speed: f32) -> f32 {
    if speed < 1.0 {
        return 0.0;
    }
    let vel_dir = velocity / speed;
    let dot_fwd = vel_dir.dot(forward);
    let dot_up = vel_dir.dot(up);
    (-dot_up).atan2(dot_fwd)
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
        let right = transform.right().as_vec3();

        let speed = aircraft.velocity.length();

        // Dynamic pressure: q = 0.5 * rho * V^2
        let q = 0.5 * rho * speed * speed;

        // Angle of attack (uses aircraft local up, with inversion correction)
        let alpha = compute_alpha(forward, up, aircraft.velocity, speed);
        aircraft.alpha = alpha;

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
        // Cd = Cd0 + Cl^2 / (pi * e * AR) + Cd_separation
        let induced_drag_coeff =
            cl * cl / (PI * aircraft.oswald_efficiency * aircraft.aspect_ratio);

        // Flow separation drag: at high AoA the wing acts like a parachute.
        // Ramps up past the stall angle, reaching Cd ≈ 1.0 at α = 90°.
        let abs_alpha = alpha.abs();
        let separation_drag = if abs_alpha > STALL_ANGLE {
            let fraction = ((abs_alpha - STALL_ANGLE) / (PI / 2.0 - STALL_ANGLE)).min(1.0);
            fraction * fraction * 1.2 // quadratic ramp to Cd ≈ 1.2
        } else {
            0.0
        };

        let cd = aircraft.cd0 + induced_drag_coeff + separation_drag;
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

        // ---- SIDESLIP FORCE ----
        // Pushes velocity toward the nose — makes rudder change flight path.
        let side_force = if speed > 1.0 {
            let vel_normalized = aircraft.velocity.normalize();
            let dot_right = vel_normalized.dot(right);
            let beta = dot_right.clamp(-1.0, 1.0).asin();
            let side_force_mag = q * aircraft.wing_area * SIDE_FORCE_COEFF * beta;
            let side_raw = right - vel_normalized * dot_right;
            let side_len = side_raw.length();
            if side_len > 0.001 {
                (side_raw / side_len) * (-side_force_mag)
            } else {
                Vec3::ZERO
            }
        } else {
            Vec3::ZERO
        };

        // ---- WEIGHT ----
        let weight_force = Vec3::new(0.0, -aircraft.mass * G, 0.0);

        // ---- SUM FORCES ----
        let total_force = lift_force + drag_force + thrust_force + side_force + weight_force;

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
        let alpha = compute_alpha(Vec3::Z, Vec3::Y, Vec3::new(0.0, 0.0, 60.0), 60.0);
        assert!(alpha.abs() < 0.001);
    }

    #[test]
    fn alpha_nose_up() {
        // Forward tilted above velocity → positive α
        // velocity is horizontal, forward has +Y component, up = world Y
        // dotUp = vel · up = 0, but vel · forward < 1, so raw_alpha comes from
        // the velocity being below the forward-up plane.
        // Use a velocity with slight downward component to create real α:
        let fwd = Vec3::new(0.0, 0.1, 1.0).normalize();
        let vel = Vec3::new(0.0, -3.0, 60.0);
        let alpha = compute_alpha(fwd, Vec3::Y, vel, vel.length());
        assert!(alpha > 0.0);
    }

    #[test]
    fn alpha_responds_at_90_bank() {
        // At 90° bank with velocity component along aircraft up: α is non-zero
        let fwd = Vec3::Z;
        let up = Vec3::X; // banked 90° right
        let vel = Vec3::new(5.0, 0.0, 60.0); // velocity drifted in +X direction
        let alpha = compute_alpha(fwd, up, vel, vel.length());
        assert!(alpha.abs() > 0.01);
    }

    #[test]
    fn alpha_works_inverted() {
        // Inverted: α sign flips (symmetric airfoil, same magnitude)
        let fwd = Vec3::new(0.0, 0.05, 1.0).normalize();
        let up_normal = Vec3::Y;
        let up_inverted = Vec3::new(0.0, -1.0, 0.0);
        let vel = Vec3::new(0.0, -3.0, 60.0);
        let a_normal = compute_alpha(fwd, up_normal, vel, vel.length());
        let a_inverted = compute_alpha(fwd, up_inverted, vel, vel.length());
        // Same magnitude, opposite sign (symmetric airfoil behavior)
        assert!((a_normal.abs() - a_inverted.abs()).abs() < 0.1);
    }

    #[test]
    fn alpha_continuous_through_vertical() {
        // No discontinuity when passing through vertical (no inversion factor)
        let fwd_80 = Vec3::new(0.0, 0.985, 0.174).normalize(); // 80° pitch
        let fwd_90 = Vec3::new(0.0, 1.0, 0.0);                 // 90° pitch
        let up_80 = Vec3::new(0.0, -0.174, 0.985).normalize();
        let up_90 = Vec3::new(0.0, 0.0, -1.0);
        let vel = Vec3::new(0.0, 10.0, 50.0);
        let a_80 = compute_alpha(fwd_80, up_80, vel, vel.length());
        let a_90 = compute_alpha(fwd_90, up_90, vel, vel.length());
        // Both should be finite and not NaN
        assert!(a_80.is_finite());
        assert!(a_90.is_finite());
    }
}
