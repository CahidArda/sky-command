use bevy::prelude::*;
use std::collections::HashSet;

use crate::aircraft::Aircraft;

/// Size of each terrain chunk in meters.
const CHUNK_SIZE: f32 = 500.0;
/// How many chunks to keep loaded around the aircraft in each direction.
const VIEW_RADIUS: i32 = 6;
/// Runway chunk stays special at (0, 0).
const RUNWAY_CHUNK: (i32, i32) = (0, 0);

/// Resource tracking which chunks are currently loaded.
#[derive(Resource, Default)]
pub struct LoadedChunks {
    pub chunks: HashSet<(i32, i32)>,
}

/// Component marking a terrain chunk entity, storing its grid coordinates.
#[derive(Component)]
pub struct TerrainChunk {
    pub cx: i32,
    pub cz: i32,
}

/// Deterministic hash from chunk coordinates.
fn chunk_seed(cx: i32, cz: i32) -> u32 {
    let mut h = (cx as u32).wrapping_mul(374761393);
    h = h.wrapping_add((cz as u32).wrapping_mul(668265263));
    h ^= h >> 13;
    h = h.wrapping_mul(1274126177);
    h ^= h >> 16;
    h
}

/// Simple seeded PRNG. Returns values in [0, 1).
fn srand(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(16807).wrapping_add(1);
    (*seed as f32) / (u32::MAX as f32)
}

/// Shared materials stored as a resource.
#[derive(Resource)]
pub struct TerrainMaterials {
    ground: Handle<StandardMaterial>,
    ground_dark: Handle<StandardMaterial>,
    runway: Handle<StandardMaterial>,
    runway_line: Handle<StandardMaterial>,
    trunk: Handle<StandardMaterial>,
    leaf: Handle<StandardMaterial>,
    building_colors: Vec<Handle<StandardMaterial>>,
}

/// Shared meshes stored as a resource.
#[derive(Resource)]
pub struct TerrainMeshes {
    ground_tile: Handle<Mesh>,
    trunk: Handle<Mesh>,
    canopy: Handle<Mesh>,
    runway: Handle<Mesh>,
    dash: Handle<Mesh>,
}

/// Initialize terrain resources.
pub fn init_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(TerrainMaterials {
        ground: materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.48, 0.25),
            perceptual_roughness: 0.9,
            ..default()
        }),
        ground_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.40, 0.20),
            perceptual_roughness: 0.95,
            ..default()
        }),
        runway: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.35, 0.35),
            perceptual_roughness: 0.8,
            ..default()
        }),
        runway_line: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        }),
        trunk: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.22, 0.10),
            ..default()
        }),
        leaf: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.42, 0.12),
            ..default()
        }),
        building_colors: vec![
            materials.add(Color::srgb(0.55, 0.55, 0.55)),
            materials.add(Color::srgb(0.50, 0.50, 0.53)),
            materials.add(Color::srgb(0.60, 0.58, 0.55)),
            materials.add(Color::srgb(0.45, 0.45, 0.48)),
            materials.add(Color::srgb(0.65, 0.60, 0.55)),
        ],
    });

    commands.insert_resource(TerrainMeshes {
        ground_tile: meshes.add(Plane3d::default().mesh().size(CHUNK_SIZE, CHUNK_SIZE)),
        trunk: meshes.add(Cuboid::new(0.6, 4.0, 0.6)),
        canopy: meshes.add(Cuboid::new(3.0, 4.0, 3.0)),
        runway: meshes.add(Plane3d::default().mesh().size(30.0, CHUNK_SIZE)),
        dash: meshes.add(Plane3d::default().mesh().size(0.5, 15.0)),
    });

    commands.insert_resource(LoadedChunks::default());
}

/// Each frame: spawn chunks near the aircraft, despawn distant ones.
pub fn update_terrain(
    mut commands: Commands,
    aircraft_query: Query<&Transform, With<Aircraft>>,
    chunk_query: Query<(Entity, &TerrainChunk)>,
    mut loaded: ResMut<LoadedChunks>,
    mats: Res<TerrainMaterials>,
    tile_meshes: Res<TerrainMeshes>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok(aircraft_tf) = aircraft_query.get_single() else {
        return;
    };

    let pos = aircraft_tf.translation;
    let center_cx = (pos.x / CHUNK_SIZE).round() as i32;
    let center_cz = (pos.z / CHUNK_SIZE).round() as i32;

    // Which chunks should exist
    let mut desired: HashSet<(i32, i32)> = HashSet::new();
    for dx in -VIEW_RADIUS..=VIEW_RADIUS {
        for dz in -VIEW_RADIUS..=VIEW_RADIUS {
            desired.insert((center_cx + dx, center_cz + dz));
        }
    }

    // Despawn chunks outside range
    for (entity, chunk) in chunk_query.iter() {
        let key = (chunk.cx, chunk.cz);
        if !desired.contains(&key) {
            commands.entity(entity).despawn_recursive();
            loaded.chunks.remove(&key);
        }
    }

    // Spawn missing chunks
    for &(cx, cz) in &desired {
        if loaded.chunks.contains(&(cx, cz)) {
            continue;
        }
        loaded.chunks.insert((cx, cz));
        spawn_chunk(&mut commands, cx, cz, &mats, &tile_meshes, &mut meshes);
    }
}

/// Spawn a single terrain chunk at grid position (cx, cz).
fn spawn_chunk(
    commands: &mut Commands,
    cx: i32,
    cz: i32,
    mats: &TerrainMaterials,
    tile_meshes: &TerrainMeshes,
    meshes: &mut Assets<Mesh>,
) {
    let world_x = cx as f32 * CHUNK_SIZE;
    let world_z = cz as f32 * CHUNK_SIZE;
    let mut seed = chunk_seed(cx, cz);
    let half = CHUNK_SIZE * 0.45;
    let is_runway = (cx, cz) == RUNWAY_CHUNK;

    commands
        .spawn((
            TerrainChunk { cx, cz },
            Mesh3d(tile_meshes.ground_tile.clone()),
            MeshMaterial3d(mats.ground.clone()),
            Transform::from_xyz(world_x, 0.0, world_z),
        ))
        .with_children(|parent| {
            // Darker ground patch for variety
            if srand(&mut seed) > 0.5 {
                let px = (srand(&mut seed) - 0.5) * CHUNK_SIZE * 0.6;
                let pz = (srand(&mut seed) - 0.5) * CHUNK_SIZE * 0.6;
                let ps = 30.0 + srand(&mut seed) * 80.0;
                parent.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(ps, ps))),
                    MeshMaterial3d(mats.ground_dark.clone()),
                    Transform::from_xyz(px, 0.08, pz),
                ));
            }

            // Runway in the origin chunk
            if is_runway {
                parent.spawn((
                    Mesh3d(tile_meshes.runway.clone()),
                    MeshMaterial3d(mats.runway.clone()),
                    Transform::from_xyz(0.0, 0.15, 0.0),
                ));
                for i in 0..12 {
                    parent.spawn((
                        Mesh3d(tile_meshes.dash.clone()),
                        MeshMaterial3d(mats.runway_line.clone()),
                        Transform::from_xyz(0.0, 0.16, -220.0 + i as f32 * 40.0),
                    ));
                }
            }

            // Trees
            let num_trees = 3 + (srand(&mut seed) * 8.0) as usize;
            for _ in 0..num_trees {
                let tx = (srand(&mut seed) - 0.5) * half * 2.0;
                let tz = (srand(&mut seed) - 0.5) * half * 2.0;
                if is_runway && tx.abs() < 20.0 {
                    continue;
                }
                parent.spawn((
                    Mesh3d(tile_meshes.trunk.clone()),
                    MeshMaterial3d(mats.trunk.clone()),
                    Transform::from_xyz(tx, 2.0, tz),
                ));
                parent.spawn((
                    Mesh3d(tile_meshes.canopy.clone()),
                    MeshMaterial3d(mats.leaf.clone()),
                    Transform::from_xyz(tx, 5.5, tz),
                ));
            }

            // Buildings (35% of chunks)
            if srand(&mut seed) > 0.65 {
                let n = 1 + (srand(&mut seed) * 4.0) as usize;
                for i in 0..n {
                    let bx = (srand(&mut seed) - 0.5) * half * 1.5;
                    let bz = (srand(&mut seed) - 0.5) * half * 1.5;
                    if is_runway && bx.abs() < 30.0 {
                        continue;
                    }
                    let h = 8.0 + srand(&mut seed) * 25.0;
                    let w = 6.0 + srand(&mut seed) * 12.0;
                    let d = 6.0 + srand(&mut seed) * 12.0;
                    parent.spawn((
                        Mesh3d(meshes.add(Cuboid::new(w, h, d))),
                        MeshMaterial3d(
                            mats.building_colors[i % mats.building_colors.len()].clone(),
                        ),
                        Transform::from_xyz(bx, h / 2.0, bz),
                    ));
                }
            }
        });
}
