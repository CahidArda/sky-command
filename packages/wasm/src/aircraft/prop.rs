use bevy::prelude::*;
use std::f32::consts::PI;

use super::{Aircraft, ControlInput, Propeller};

/// Prop plane specifications matching a Cessna 172-like aircraft.
pub fn default_aircraft() -> Aircraft {
    Aircraft {
        velocity: Vec3::new(0.0, 0.0, 60.0),
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
///
/// The mesh is modeled with the nose at -Z (Bevy's forward convention).
/// Nose/propeller face -Z, tail faces +Z.
///
/// Realistic Cessna 172 proportions:
///   Wingspan ~11m, fuselage length ~8.3m, height ~2.7m
///   HIGH WING with diagonal struts, tapered fuselage, tricycle gear.
pub fn spawn_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Materials ──────────────────────────────────────────────────────
    let white_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.95, 0.95),
        ..default()
    });
    let off_white_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.88, 0.88, 0.88),
        ..default()
    });
    let wing_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.92, 0.93),
        ..default()
    });
    let cowling_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.75, 0.78),
        ..default()
    });
    let dark_grey_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.35, 0.38),
        ..default()
    });
    let windshield_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.65, 0.8, 0.45),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.12, 0.08),
        ..default()
    });
    let accent2_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.25, 0.55),
        ..default()
    });
    let prop_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.10, 0.05),
        ..default()
    });
    let gear_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.18, 0.20),
        ..default()
    });
    let tire_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.12, 0.12),
        ..default()
    });

    // ── Fuselage reference frame ───────────────────────────────────────
    // Total fuselage: ~8.3m along Z.  Nose at roughly Z = -4.15, tail at Z = +4.15.
    // We place the root entity at the CG, which for a C172 is roughly 30% MAC.
    // Wings are at Z ≈ -0.6 (slightly ahead of center), CG under the wings.
    // The root cuboid is the main cabin section.
    //
    // Y = 0 is the fuselage centerline. Bottom of fuselage at Y ≈ -0.55.
    // Wing top surface at Y ≈ +0.65 (high wing).

    let root_mesh = meshes.add(Cuboid::new(1.10, 1.10, 2.80));

    let trim_alpha: f32 = 0.053;
    let start_transform = Transform::from_xyz(0.0, 1000.0, 0.0)
        .with_rotation(
            Quat::from_rotation_y(PI) * Quat::from_rotation_x(trim_alpha),
        );

    commands
        .spawn((
            default_aircraft(),
            ControlInput::default(),
            Mesh3d(root_mesh),
            MeshMaterial3d(white_mat.clone()),
            start_transform,
        ))
        .with_children(|parent| {
            // ── FUSELAGE SECTIONS (tapered, nose at -Z) ────────────────

            // Forward cabin / firewall area — slightly narrower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.00, 1.00, 0.80))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -1.80),
            ));

            // Nose cowling — engine housing, wider & shorter
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.90, 0.85, 1.00))),
                MeshMaterial3d(cowling_mat.clone()),
                Transform::from_xyz(0.0, -0.05, -2.70),
            ));

            // Cowling front cap — tapers slightly
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.75, 0.72, 0.30))),
                MeshMaterial3d(cowling_mat.clone()),
                Transform::from_xyz(0.0, -0.05, -3.35),
            ));

            // Spinner cone (nose tip)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 0.30, 0.25))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.0, -0.05, -3.62),
            ));

            // Aft fuselage section 1 — starts to taper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.95, 0.95, 1.20))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, 2.00),
            ));

            // Aft fuselage section 2 — narrower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.72, 0.72, 1.00))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.05, 3.10),
            ));

            // Tail cone — final taper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.45, 0.45, 0.80))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(0.0, 0.10, 3.95),
            ));

            // ── FUSELAGE ACCENT STRIPE (red) along the side ───────────
            // Runs along the fuselage at mid-height on both sides
            let stripe_y = 0.05;
            let stripe_thickness = 0.02;
            let stripe_height = 0.12;

            // Left stripe segments
            for &(z, half_len, half_w) in &[
                (-2.70, 0.50, 0.46),
                (-1.80, 0.40, 0.51),
                (0.0, 1.40, 0.56),
                (2.00, 0.60, 0.49),
                (3.10, 0.50, 0.37),
            ] {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        stripe_height,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent_mat.clone()),
                    Transform::from_xyz(-half_w, stripe_y, z),
                ));
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        stripe_height,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent_mat.clone()),
                    Transform::from_xyz(half_w, stripe_y, z),
                ));
            }

            // Thinner blue accent stripe just below the red one
            for &(z, half_len, half_w) in &[
                (-2.70, 0.50, 0.46),
                (-1.80, 0.40, 0.51),
                (0.0, 1.40, 0.56),
                (2.00, 0.60, 0.49),
            ] {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        0.06,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent2_mat.clone()),
                    Transform::from_xyz(-half_w, stripe_y - 0.10, z),
                ));
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        0.06,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent2_mat.clone()),
                    Transform::from_xyz(half_w, stripe_y - 0.10, z),
                ));
            }

            // ── CABIN / WINDSHIELD (raised section on top) ─────────────
            // The C172 has a distinctive tall cabin with large windows.

            // Cabin roof — sits on top of the main fuselage
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.95, 0.30, 1.80))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.70, -0.30),
            ));

            // Windshield — slanted forward, approximated with a rotated cuboid
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.90, 0.45, 0.04))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(0.0, 0.68, -1.18)
                    .with_rotation(Quat::from_rotation_x(-0.45)),
            ));

            // Rear window — slightly angled back
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 0.35, 0.04))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(0.0, 0.68, 0.62)
                    .with_rotation(Quat::from_rotation_x(0.35)),
            ));

            // Side windows (left)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, 0.35, 1.30))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(-0.48, 0.55, -0.35),
            ));

            // Side windows (right)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, 0.35, 1.30))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(0.48, 0.55, -0.35),
            ));

            // ── HIGH WINGS ─────────────────────────────────────────────
            // Mounted on top of the fuselage, Y ≈ 0.65 (top of cabin).
            // Wingspan 11m, chord ~1.6m, slight dihedral (1.5°).
            let wing_y = 0.85;
            let wing_z = -0.50; // slightly forward of CG

            // Left wing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.80, 0.14, 1.55))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-2.90, wing_y, wing_z)
                    .with_rotation(Quat::from_rotation_z(1.5_f32.to_radians())),
            ));

            // Right wing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.80, 0.14, 1.55))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(2.90, wing_y, wing_z)
                    .with_rotation(Quat::from_rotation_z(-1.5_f32.to_radians())),
            ));

            // Wing root fairing — blends wing into fuselage top
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.10, 0.12, 1.60))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(0.0, wing_y - 0.02, wing_z),
            ));

            // Wing tips (slightly thinner at the ends)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.40, 0.10, 1.20))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-5.40, wing_y + 0.10, wing_z),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.40, 0.10, 1.20))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(5.40, wing_y + 0.10, wing_z),
            ));

            // Wing tip accent (red on trailing edge tips)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 0.11, 0.40))),
                MeshMaterial3d(accent_mat.clone()),
                Transform::from_xyz(-5.45, wing_y + 0.10, wing_z + 0.50),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 0.11, 0.40))),
                MeshMaterial3d(accent_mat.clone()),
                Transform::from_xyz(5.45, wing_y + 0.10, wing_z + 0.50),
            ));

            // ── WING STRUTS (diagonal, from lower fuselage up to wing) ─
            // Each strut runs from roughly (±0.55, -0.15, wing_z+0.2) on the
            // fuselage to (±2.5, wing_y-0.07, wing_z) on the wing underside.
            // Length ≈ sqrt((2.5-0.55)^2 + (0.85+0.15)^2) ≈ 2.2m
            // Angle ≈ atan2(1.0, 1.95) ≈ 27° from horizontal
            let strut_len = 2.20;
            let strut_angle = (1.00_f32).atan2(1.95); // ~27°

            // Left strut (front)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.04, strut_len, 0.06))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-1.52, 0.35, wing_z + 0.10)
                    .with_rotation(Quat::from_rotation_z(strut_angle)),
            ));

            // Left strut (rear/jury strut) — thinner, slightly aft
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, strut_len * 0.95, 0.04))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-1.52, 0.35, wing_z + 0.55)
                    .with_rotation(Quat::from_rotation_z(strut_angle)),
            ));

            // Right strut (front)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.04, strut_len, 0.06))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(1.52, 0.35, wing_z + 0.10)
                    .with_rotation(Quat::from_rotation_z(-strut_angle)),
            ));

            // Right strut (rear/jury strut)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, strut_len * 0.95, 0.04))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(1.52, 0.35, wing_z + 0.55)
                    .with_rotation(Quat::from_rotation_z(-strut_angle)),
            ));

            // ── HORIZONTAL STABILIZER (tail, low) ──────────────────────
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.40, 0.07, 1.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(0.0, 0.20, 4.10),
            ));

            // ── VERTICAL STABILIZER with dorsal fin ────────────────────
            // Main fin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 1.40, 1.10))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.95, 3.90),
            ));

            // Upper fin tip — slightly narrower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.07, 0.40, 0.60))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 1.80, 3.70),
            ));

            // Dorsal fin — the fillet between fuselage and vertical stab
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.35, 0.50))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(0.0, 0.40, 3.50),
            ));

            // Rudder accent stripe (on vertical stab)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 0.25, 0.80))),
                MeshMaterial3d(accent_mat.clone()),
                Transform::from_xyz(0.0, 1.55, 4.00),
            ));

            // ── PROPELLER (nose at -Z) ─────────────────────────────────
            // Two-blade prop, ~1.9m diameter
            parent.spawn((
                Propeller,
                Mesh3d(meshes.add(Cuboid::new(1.90, 0.18, 0.06))),
                MeshMaterial3d(prop_mat),
                Transform::from_xyz(0.0, -0.05, -3.78),
            ));

            // ── TRICYCLE LANDING GEAR ───────────────────────────────────
            // Nosewheel at front, two mains under the wings.
            let wheel_mesh = meshes.add(Cylinder::new(0.18, 0.10));

            // -- Nosewheel strut --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.70, 0.06))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(0.0, -0.90, -2.60),
            ));
            // Nosewheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(0.0, -1.28, -2.60)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // -- Left main gear --
            // V-shaped strut from fuselage down to wheel
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.85, 0.08))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-0.80, -0.95, -0.20)
                    .with_rotation(Quat::from_rotation_z(-0.20)),
            ));
            // Axle / horizontal brace
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.50, 0.05, 0.06))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-1.00, -1.35, -0.20),
            ));
            // Left main wheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-1.15, -1.35, -0.20)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
            // Left wheel fairing/pant
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 0.25, 0.45))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(-1.15, -1.32, -0.20),
            ));

            // -- Right main gear --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.85, 0.08))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(0.80, -0.95, -0.20)
                    .with_rotation(Quat::from_rotation_z(0.20)),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.50, 0.05, 0.06))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(1.00, -1.35, -0.20),
            ));
            // Right main wheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(1.15, -1.35, -0.20)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
            // Right wheel fairing/pant
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 0.25, 0.45))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(1.15, -1.32, -0.20),
            ));

            // Nosewheel fairing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.14, 0.22, 0.35))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, -1.25, -2.60),
            ));
        });
}
