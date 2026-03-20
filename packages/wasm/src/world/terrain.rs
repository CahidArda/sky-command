use bevy::prelude::*;

/// Simple seeded pseudo-random for deterministic placement.
fn seeded_random(seed: u32) -> impl FnMut() -> f32 {
    let mut s = seed;
    move || {
        s = s.wrapping_mul(16807).wrapping_add(1);
        (s as f32) / (u32::MAX as f32)
    }
}

/// Spawn the terrain: ground plane, runway, trees, and buildings.
pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Ground plane ─────────────────────────────────────────────────
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.48, 0.25),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // ── Grid markers (darker patches every 500m) ─────────────────────
    let marker_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.40, 0.20),
        perceptual_roughness: 0.95,
        ..default()
    });
    let marker_mesh = meshes.add(Plane3d::default().mesh().size(50.0, 50.0));
    for x in (-10..=10).map(|i| i as f32 * 500.0) {
        for z in (-10..=10).map(|i| i as f32 * 500.0) {
            commands.spawn((
                Mesh3d(marker_mesh.clone()),
                MeshMaterial3d(marker_mat.clone()),
                Transform::from_xyz(x, 0.01, z),
            ));
        }
    }

    // ── Runway with markings ─────────────────────────────────────────
    let runway_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.35, 0.35),
        perceptual_roughness: 0.8,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 800.0))),
        MeshMaterial3d(runway_mat),
        Transform::from_xyz(0.0, 0.15, 0.0),
    ));

    // Center line dashes
    let white_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.9),
        ..default()
    });
    let dash_mesh = meshes.add(Plane3d::default().mesh().size(0.5, 15.0));
    for i in 0..20 {
        commands.spawn((
            Mesh3d(dash_mesh.clone()),
            MeshMaterial3d(white_mat.clone()),
            Transform::from_xyz(0.0, 0.16, -380.0 + i as f32 * 40.0),
        ));
    }

    // Threshold markings
    let thresh_mesh = meshes.add(Plane3d::default().mesh().size(1.5, 20.0));
    for &z in &[-390.0_f32, 390.0] {
        for &x in &[-8.0_f32, -4.0, 0.0, 4.0, 8.0] {
            commands.spawn((
                Mesh3d(thresh_mesh.clone()),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(x, 0.17, z),
            ));
        }
    }

    // ── Trees ────────────────────────────────────────────────────────
    let trunk_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.22, 0.10),
        ..default()
    });
    let leaf_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.42, 0.12),
        ..default()
    });
    let trunk_mesh = meshes.add(Cuboid::new(0.6, 4.0, 0.6));
    let canopy_mesh = meshes.add(Cuboid::new(3.0, 4.0, 3.0));

    let mut rng = seeded_random(42);
    for _ in 0..400 {
        let x = (rng() - 0.5) * 8000.0;
        let z = (rng() - 0.5) * 8000.0;
        // Skip runway area
        if x.abs() < 40.0 && z.abs() < 500.0 {
            continue;
        }
        // Trunk
        commands.spawn((
            Mesh3d(trunk_mesh.clone()),
            MeshMaterial3d(trunk_mat.clone()),
            Transform::from_xyz(x, 2.0, z),
        ));
        // Canopy
        commands.spawn((
            Mesh3d(canopy_mesh.clone()),
            MeshMaterial3d(leaf_mat.clone()),
            Transform::from_xyz(x, 5.5, z),
        ));
    }

    // ── Buildings ────────────────────────────────────────────────────
    let building_colors = [
        Color::srgb(0.55, 0.55, 0.55),
        Color::srgb(0.50, 0.50, 0.53),
        Color::srgb(0.60, 0.58, 0.55),
        Color::srgb(0.65, 0.63, 0.60),
        Color::srgb(0.45, 0.45, 0.48),
    ];
    let building_mats: Vec<_> = building_colors
        .iter()
        .map(|&c| {
            materials.add(StandardMaterial {
                base_color: c,
                ..default()
            })
        })
        .collect();

    let mut rng2 = seeded_random(123);
    for i in 0..60 {
        let x = (rng2() - 0.5) * 3000.0;
        let z = (rng2() - 0.5) * 3000.0;
        if x.abs() < 50.0 && z.abs() < 500.0 {
            continue;
        }
        let h = 8.0 + rng2() * 30.0;
        let w = 6.0 + (i % 5) as f32 * 3.0;
        let d = 6.0 + ((i * 7) % 5) as f32 * 3.0;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(w, h, d))),
            MeshMaterial3d(building_mats[i % building_mats.len()].clone()),
            Transform::from_xyz(x, h / 2.0, z),
        ));
    }
}
