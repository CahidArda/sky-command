use bevy::prelude::*;

use crate::aircraft::Aircraft;
use crate::physics::PhysicsSet;

/// Marker component for the chase camera.
#[derive(Component)]
pub struct ChaseCamera {
    /// Offset behind the aircraft in local space (x, y, z).
    /// Positive Z = behind, positive Y = above.
    pub offset: Vec3,
    /// How quickly the camera catches up (0.0 = no movement, 1.0 = instant).
    pub smoothing: f32,
}

impl Default for ChaseCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 8.0, 25.0),
            smoothing: 3.0,
        }
    }
}

pub struct CameraPlugin;

/// System set for camera updates — runs after transform sync.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSet;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            CameraSet.after(PhysicsSet::TransformSync),
        )
        .add_systems(Startup, spawn_chase_camera)
        .add_systems(Update, update_chase_camera.in_set(CameraSet));
    }
}

/// Spawn the chase camera.
fn spawn_chase_camera(mut commands: Commands) {
    commands.spawn((
        ChaseCamera::default(),
        Camera3d::default(),
        Transform::from_xyz(0.0, 1008.0, -25.0).looking_at(Vec3::new(0.0, 1000.0, 0.0), Vec3::Y),
    ));
}

/// Update the chase camera to follow the aircraft with smooth interpolation.
fn update_chase_camera(
    time: Res<Time>,
    aircraft_query: Query<&Transform, (With<Aircraft>, Without<ChaseCamera>)>,
    mut camera_query: Query<(&ChaseCamera, &mut Transform), Without<Aircraft>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let Ok(aircraft_transform) = aircraft_query.get_single() else {
        return;
    };

    for (chase_cam, mut cam_transform) in camera_query.iter_mut() {
        // Compute desired camera position in world space.
        // The offset is in the aircraft's local space, so we rotate it.
        let desired_offset = aircraft_transform.rotation * chase_cam.offset;
        let desired_position = aircraft_transform.translation + desired_offset;

        // Smooth interpolation toward desired position
        let t = (chase_cam.smoothing * dt).min(1.0);
        cam_transform.translation = cam_transform.translation.lerp(desired_position, t);

        // Always look at the aircraft
        cam_transform.look_at(aircraft_transform.translation, Vec3::Y);
    }
}
