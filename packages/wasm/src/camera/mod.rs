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
        app.configure_sets(
            Update,
            CameraSet.after(PhysicsSet::TransformSync),
        )
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
fn toggle_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut FlightCamera>,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        for mut cam in query.iter_mut() {
            cam.mode = match cam.mode {
                CameraMode::Chase => CameraMode::Cockpit,
                CameraMode::Cockpit => CameraMode::Chase,
            };
        }
    }
}

/// Update camera position based on mode.
fn update_flight_camera(
    time: Res<Time>,
    aircraft_query: Query<&Transform, (With<Aircraft>, Without<FlightCamera>)>,
    mut camera_query: Query<(&FlightCamera, &mut Transform), Without<Aircraft>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let Ok(aircraft_transform) = aircraft_query.get_single() else {
        return;
    };

    for (cam, mut cam_transform) in camera_query.iter_mut() {
        match cam.mode {
            CameraMode::Chase => {
                let desired_offset = aircraft_transform.rotation * cam.chase_offset;
                let desired_position = aircraft_transform.translation + desired_offset;

                let t = (cam.smoothing * dt).min(1.0);
                cam_transform.translation = cam_transform.translation.lerp(desired_position, t);
                cam_transform.look_at(aircraft_transform.translation, Vec3::Y);
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
