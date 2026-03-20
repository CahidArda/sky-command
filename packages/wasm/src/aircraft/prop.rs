use bevy::prelude::*;
use std::f32::consts::PI;

use super::{Aircraft, ControlInput, Propeller};

/// Prop plane specifications matching a Cessna 172-like aircraft.
pub fn default_aircraft() -> Aircraft {
    Aircraft {
        velocity: Vec3::new(0.0, 0.0, 60.0), // ~60 m/s cruise speed, heading north (+Z)
        throttle: 0.6,
        angular_velocity: Vec3::ZERO,
        mass: 1111.0,
        wing_area: 16.2,
        max_thrust: 3500.0,
        cd0: 0.027,
        oswald_efficiency: 0.8,
        aspect_ratio: 7.32,
        pitch_rate: 60.0_f32.to_radians(),
        roll_rate: 90.0_f32.to_radians(),
        yaw_rate: 30.0_f32.to_radians(),
    }
}

/// Spawn the prop plane aircraft at altitude 1000m heading north.
pub fn spawn_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let fuselage_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.85),
        ..default()
    });
    let wing_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.7, 0.75),
        ..default()
    });
    let tail_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.65),
        ..default()
    });
    let prop_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.2, 0.1),
        ..default()
    });
    let accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.15, 0.1),
        ..default()
    });

    // Fuselage: elongated box
    let fuselage_mesh = meshes.add(Cuboid::new(1.0, 1.0, 6.0));
    // Main wings: wide thin box
    let wing_mesh = meshes.add(Cuboid::new(11.0, 0.1, 1.5));
    // Horizontal stabilizer
    let h_stab_mesh = meshes.add(Cuboid::new(3.5, 0.08, 0.8));
    // Vertical stabilizer
    let v_stab_mesh = meshes.add(Cuboid::new(0.08, 1.2, 0.9));
    // Propeller disc
    let prop_mesh = meshes.add(Cuboid::new(2.5, 0.15, 0.08));
    // Nose cone
    let nose_mesh = meshes.add(Cuboid::new(0.6, 0.6, 0.8));
    // Wing struts
    let strut_mesh = meshes.add(Cuboid::new(0.05, 0.8, 0.05));

    // Starting position: altitude 1000m, heading north (+Z direction).
    // Bevy's forward is -Z, so rotate PI around Y to face +Z (north).
    // Add a slight nose-up pitch (~3°) so the aircraft starts at trim α
    // where Lift ≈ Weight. Without this, α=0 → Cl=0 → no lift, and
    // banking (tilting lift) has no effect.
    let trim_alpha: f32 = 0.053; // ~3 degrees
    let start_transform = Transform::from_xyz(0.0, 1000.0, 0.0)
        .with_rotation(
            Quat::from_rotation_y(PI) * Quat::from_rotation_x(trim_alpha),
        );

    commands
        .spawn((
            default_aircraft(),
            ControlInput::default(),
            Mesh3d(fuselage_mesh),
            MeshMaterial3d(fuselage_mat.clone()),
            start_transform,
        ))
        .with_children(|parent| {
            // Main wings — positioned slightly forward
            parent.spawn((
                Mesh3d(wing_mesh),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(0.0, 0.1, 0.3),
            ));

            // Wing tip accents (left)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.8, 0.12, 1.5))),
                MeshMaterial3d(accent_mat.clone()),
                Transform::from_xyz(-5.5, 0.1, 0.3),
            ));

            // Wing tip accents (right)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.8, 0.12, 1.5))),
                MeshMaterial3d(accent_mat.clone()),
                Transform::from_xyz(5.5, 0.1, 0.3),
            ));

            // Left wing strut
            parent.spawn((
                Mesh3d(strut_mesh.clone()),
                MeshMaterial3d(tail_mat.clone()),
                Transform::from_xyz(-2.0, -0.3, 0.3),
            ));

            // Right wing strut
            parent.spawn((
                Mesh3d(strut_mesh),
                MeshMaterial3d(tail_mat.clone()),
                Transform::from_xyz(2.0, -0.3, 0.3),
            ));

            // Horizontal stabilizer — at the tail
            parent.spawn((
                Mesh3d(h_stab_mesh),
                MeshMaterial3d(wing_mat),
                Transform::from_xyz(0.0, 0.2, -2.8),
            ));

            // Vertical stabilizer — at the tail, pointing up
            parent.spawn((
                Mesh3d(v_stab_mesh),
                MeshMaterial3d(tail_mat.clone()),
                Transform::from_xyz(0.0, 0.9, -2.6),
            ));

            // Tail accent stripe
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.3, 0.9))),
                MeshMaterial3d(accent_mat),
                Transform::from_xyz(0.0, 1.35, -2.6),
            ));

            // Nose cone
            parent.spawn((
                Mesh3d(nose_mesh),
                MeshMaterial3d(tail_mat),
                Transform::from_xyz(0.0, 0.0, 3.3),
            ));

            // Propeller — at the nose, spinning
            parent.spawn((
                Propeller,
                Mesh3d(prop_mesh),
                MeshMaterial3d(prop_mat),
                Transform::from_xyz(0.0, 0.0, 3.8),
            ));

            // Landing gear (simple boxes underneath)
            let gear_mat = materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.2, 0.2),
                ..default()
            });
            let gear_mesh = meshes.add(Cuboid::new(0.1, 0.6, 0.1));
            let wheel_mesh = meshes.add(Cylinder::new(0.15, 0.08));

            // Left gear
            parent.spawn((
                Mesh3d(gear_mesh.clone()),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-1.2, -0.8, 0.5),
            ));
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-1.2, -1.1, 0.5)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // Right gear
            parent.spawn((
                Mesh3d(gear_mesh.clone()),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(1.2, -0.8, 0.5),
            ));
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(1.2, -1.1, 0.5)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // Tail wheel
            parent.spawn((
                Mesh3d(gear_mesh),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(0.0, -0.6, -2.5),
            ));
            parent.spawn((
                Mesh3d(wheel_mesh),
                MeshMaterial3d(gear_mat),
                Transform::from_xyz(0.0, -0.85, -2.5)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
        });
}
