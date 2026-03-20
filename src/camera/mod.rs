use bevy::prelude::*;

use crate::aircraft::Aircraft;
use crate::physics::PhysicsSet;
use crate::state::GameState;

/// Camera mode: chase (behind) or cockpit (first-person).
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraMode {
    #[default]
    Chase,
    Cockpit,
}

/// Component on the camera entity.
#[derive(Component)]
pub struct FlightCamera {
    pub mode: CameraMode,
    pub chase_offset: Vec3,
    pub cockpit_offset: Vec3,
    pub smoothing: f32,
}

impl Default for FlightCamera {
    fn default() -> Self {
        Self {
            mode: CameraMode::Chase,
            chase_offset: Vec3::new(0.0, 8.0, 25.0),
            cockpit_offset: Vec3::new(0.0, 0.8, -2.5), // pilot's head, looking forward
            smoothing: 3.0,
        }
    }
}

pub struct CameraPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSet;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, CameraSet.after(PhysicsSet::TransformSync))
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (toggle_camera_mode, update_flight_camera)
                    .chain()
                    .in_set(CameraSet)
                    .run_if(in_state(GameState::Flying)),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        FlightCamera::default(),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Toggle camera mode with C key.
fn toggle_camera_mode(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut FlightCamera>) {
    if keys.just_pressed(KeyCode::KeyC) {
        for mut cam in query.iter_mut() {
            cam.mode = match cam.mode {
                CameraMode::Chase => CameraMode::Cockpit,
                CameraMode::Cockpit => CameraMode::Chase,
            };
        }
    }
}

/// How much the camera look-at point shifts toward the velocity direction.
/// Higher = more visible AoA/slip offset on screen.
const LOOK_OFFSET_SCALE: f32 = 8.0;
/// Maximum look-at offset in meters (prevents aircraft going to screen edge).
const LOOK_OFFSET_MAX: f32 = 5.0;

/// Update camera position based on mode.
fn update_flight_camera(
    time: Res<Time>,
    aircraft_query: Query<(&Aircraft, &Transform), Without<FlightCamera>>,
    mut camera_query: Query<(&FlightCamera, &mut Transform), Without<Aircraft>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let Ok((aircraft, aircraft_transform)) = aircraft_query.get_single() else {
        return;
    };

    for (cam, mut cam_transform) in camera_query.iter_mut() {
        match cam.mode {
            CameraMode::Chase => {
                let desired_offset = aircraft_transform.rotation * cam.chase_offset;
                let desired_position = aircraft_transform.translation + desired_offset;

                let t = (cam.smoothing * dt).min(1.0);
                cam_transform.translation = cam_transform.translation.lerp(desired_position, t);

                // Compute AoA/slip-based look-at offset.
                // The camera looks slightly toward where the aircraft is GOING
                // (velocity direction), so the aircraft appears shifted on screen
                // in the direction opposite to the velocity — giving visual
                // feedback for angle of attack and sideslip.
                let forward = aircraft_transform.forward().as_vec3();
                let up = aircraft_transform.up().as_vec3();
                let right = aircraft_transform.right().as_vec3();
                let speed = aircraft.velocity.length();

                let look_offset = if speed > 5.0 {
                    let vel_dir = aircraft.velocity / speed;
                    // How much velocity deviates from nose in each local axis
                    let slip = vel_dir.dot(right) - forward.dot(right);
                    let aoa = -(vel_dir.dot(up) - forward.dot(up));
                    // Clamp each component
                    let sx = slip.clamp(-1.0, 1.0) * LOOK_OFFSET_SCALE;
                    let sy = aoa.clamp(-1.0, 1.0) * LOOK_OFFSET_SCALE;
                    let offset = right * sx.clamp(-LOOK_OFFSET_MAX, LOOK_OFFSET_MAX)
                        + up * sy.clamp(-LOOK_OFFSET_MAX, LOOK_OFFSET_MAX);
                    offset
                } else {
                    Vec3::ZERO
                };

                let look_target = aircraft_transform.translation + look_offset;

                // Choose a camera up vector that avoids degeneracy.
                let look_dir = (look_target - cam_transform.translation).normalize_or_zero();
                let aircraft_up = up;
                let cam_up = if look_dir.dot(aircraft_up).abs() > 0.95 {
                    aircraft_transform.forward().as_vec3()
                } else {
                    aircraft_up
                };
                cam_transform.look_at(look_target, cam_up);
            }
            CameraMode::Cockpit => {
                // Snap to cockpit — no interpolation, locked to aircraft
                let offset = aircraft_transform.rotation * cam.cockpit_offset;
                cam_transform.translation = aircraft_transform.translation + offset;
                // Look forward from the cockpit
                let look_ahead = aircraft_transform.rotation * Vec3::new(0.0, 0.0, -100.0);
                let look_target = cam_transform.translation + look_ahead;
                cam_transform.look_at(look_target, aircraft_transform.up().as_vec3());
            }
        }
    }
}
