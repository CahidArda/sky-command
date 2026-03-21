use bevy::prelude::*;

use crate::aircraft::Aircraft;
use crate::physics::flight_model::STALL_ANGLE;

/// Marker component for a trail point entity. Contains a despawn timer.
#[derive(Component)]
pub struct TrailPoint {
    pub timer: Timer,
}

/// Threshold: AoA must exceed 80% of stall angle to generate vortex trails.
const VORTEX_AOA_THRESHOLD: f32 = STALL_ANGLE * 0.8;

/// How long each trail point persists before being despawned (seconds).
const TRAIL_LIFETIME: f32 = 2.0;

/// Minimum interval between spawning trail points (seconds).
const SPAWN_INTERVAL: f32 = 0.03;

/// Resource tracking time since last trail spawn.
#[derive(Resource)]
pub struct TrailSpawnTimer(pub Timer);

impl Default for TrailSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating))
    }
}

/// Shared trail material resource (created once).
#[derive(Resource)]
pub struct TrailResources {
    pub material: Handle<StandardMaterial>,
    pub mesh: Handle<Mesh>,
}

/// Initialize trail resources when entering Flying state.
pub fn init_trail_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.6),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let mesh = meshes.add(Cuboid::new(0.3, 0.3, 0.3));
    commands.insert_resource(TrailResources { material, mesh });
    commands.insert_resource(TrailSpawnTimer::default());
}

/// Spawn trail points at wingtip positions when AoA is high enough.
pub fn spawn_trail_points(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<TrailSpawnTimer>,
    aircraft_query: Query<(&Aircraft, &Transform)>,
    trail_res: Option<Res<TrailResources>>,
) {
    let Some(trail_res) = trail_res else {
        return;
    };

    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    for (aircraft, transform) in aircraft_query.iter() {
        if aircraft.alpha.abs() < VORTEX_AOA_THRESHOLD {
            continue;
        }

        // Wingtip positions from the aircraft's specs (set per aircraft type).
        let left_wingtip_local = aircraft.wingtip_left;
        let right_wingtip_local = aircraft.wingtip_right;

        let left_world = transform.transform_point(left_wingtip_local);
        let right_world = transform.transform_point(right_wingtip_local);

        // Spawn left wingtip trail point
        commands.spawn((
            TrailPoint {
                timer: Timer::from_seconds(TRAIL_LIFETIME, TimerMode::Once),
            },
            Mesh3d(trail_res.mesh.clone()),
            MeshMaterial3d(trail_res.material.clone()),
            Transform::from_translation(left_world),
        ));

        // Spawn right wingtip trail point
        commands.spawn((
            TrailPoint {
                timer: Timer::from_seconds(TRAIL_LIFETIME, TimerMode::Once),
            },
            Mesh3d(trail_res.mesh.clone()),
            MeshMaterial3d(trail_res.material.clone()),
            Transform::from_translation(right_world),
        ));
    }
}

/// Tick trail point timers and despawn expired ones.
pub fn despawn_expired_trail_points(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailPoint)>,
) {
    for (entity, mut trail_point) in query.iter_mut() {
        trail_point.timer.tick(time.delta());
        if trail_point.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Cleanup all trail points when leaving Flying state.
pub fn cleanup_trails(mut commands: Commands, query: Query<Entity, With<TrailPoint>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
