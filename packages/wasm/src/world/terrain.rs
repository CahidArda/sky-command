use bevy::prelude::*;

/// Spawn a large flat green ground plane.
pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main ground plane — a large flat surface
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.48, 0.25),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Add a few visual reference markers on the ground so the player
    // can perceive movement and altitude. These are simple darker patches.
    let marker_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.40, 0.20),
        perceptual_roughness: 0.95,
        ..default()
    });

    let marker_mesh = meshes.add(Plane3d::default().mesh().size(50.0, 50.0));

    // Grid of markers every 500m
    for x in (-10..=10).map(|i| i as f32 * 500.0) {
        for z in (-10..=10).map(|i| i as f32 * 500.0) {
            commands.spawn((
                Mesh3d(marker_mesh.clone()),
                MeshMaterial3d(marker_mat.clone()),
                Transform::from_xyz(x, 0.01, z),
            ));
        }
    }

    // Runway-like strip for reference
    let runway_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.35, 0.35),
        perceptual_roughness: 0.8,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 800.0))),
        MeshMaterial3d(runway_mat),
        Transform::from_xyz(0.0, 0.02, 0.0),
    ));
}
