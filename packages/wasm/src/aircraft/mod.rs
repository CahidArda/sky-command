use bevy::prelude::*;

pub mod prop;

/// Marker component for the aircraft entity.
#[derive(Component)]
pub struct Aircraft {
    /// Velocity in world space (m/s).
    pub velocity: Vec3,
    /// Throttle setting, 0.0 to 1.0.
    pub throttle: f32,
    /// Angular velocity in local space (pitch, yaw, roll) in rad/s.
    pub angular_velocity: Vec3,
    /// Aircraft mass in kg.
    pub mass: f32,
    /// Wing area in m^2.
    pub wing_area: f32,
    /// Maximum engine thrust in Newtons.
    pub max_thrust: f32,
    /// Zero-lift drag coefficient.
    pub cd0: f32,
    /// Oswald span efficiency factor.
    pub oswald_efficiency: f32,
    /// Wing aspect ratio.
    pub aspect_ratio: f32,
    /// Maximum pitch rate in rad/s.
    pub pitch_rate: f32,
    /// Maximum roll rate in rad/s.
    pub roll_rate: f32,
    /// Maximum yaw rate in rad/s.
    pub yaw_rate: f32,
}

/// Control input component attached to the aircraft.
#[derive(Component, Default)]
pub struct ControlInput {
    /// Pitch input, -1.0 (nose down) to 1.0 (nose up).
    pub pitch: f32,
    /// Roll input, -1.0 (left) to 1.0 (right).
    pub roll: f32,
    /// Yaw input, -1.0 (left) to 1.0 (right).
    pub yaw: f32,
    /// Throttle change input, -1.0 to 1.0.
    pub throttle_change: f32,
}

/// Marker for the propeller mesh so we can spin it.
#[derive(Component)]
pub struct Propeller;

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, prop::spawn_aircraft);
    }
}
