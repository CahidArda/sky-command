use bevy::prelude::*;
use std::f32::consts::PI;

use super::{Aircraft, ControlInput};

/// B-2 Spirit specifications.
pub fn default_aircraft() -> Aircraft {
    Aircraft {
        velocity: Vec3::new(0.0, 0.0, 270.0),
        throttle: 0.65,
        angular_velocity: Vec3::ZERO,
        mass: 71000.0,
        wing_area: 478.0,
        max_thrust: 308000.0,
        cd0: 0.018,
        oswald_efficiency: 0.90,
        aspect_ratio: 5.92,
        pitch_rate: 25.0_f32.to_radians(),
        roll_rate: 35.0_f32.to_radians(),
        yaw_rate: 15.0_f32.to_radians(),
        side_force_coeff: 0.3,
        alpha: 0.0,
        g_load: 1.0,    }
}

/// Spawn the B-2 Spirit stealth bomber at altitude 1000 m heading north.
///
/// The mesh is modeled with the nose at -Z (Bevy's forward convention).
/// Nose faces -Z, trailing edge at +Z.
///
/// B-2 Spirit proportions:
///   Wingspan ~52 m, length ~21 m, height ~5 m.
///   Flying wing — no conventional fuselage or vertical tail.
///   W-shaped trailing edge, four top-mounted engine intakes,
///   cockpit bubble at front center, dark grey stealth finish.
pub fn spawn_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Materials ──────────────────────────────────────────────────────
    let stealth_dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.18, 0.20),
        ..default()
    });
    let stealth_mid = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.22, 0.24),
        ..default()
    });
    let stealth_light = materials.add(StandardMaterial {
        base_color: Color::srgb(0.26, 0.26, 0.28),
        ..default()
    });
    let intake_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.10, 0.10),
        ..default()
    });
    let exhaust_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.08, 0.08),
        ..default()
    });
    let cockpit_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.35, 0.45, 0.55, 0.50),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let gear_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.17),
        ..default()
    });
    let tire_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.08, 0.08),
        ..default()
    });
    let edge_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.14, 0.14, 0.16),
        ..default()
    });

    // ── Root entity ──────────────────────────────────────────────────
    // The center body blended section — roughly 8 m wide, 21 m long, 2.5 m tall.
    let root_mesh = meshes.add(Cuboid::new(8.0, 2.0, 16.0));

    let trim_alpha: f32 = 0.004;
    let start_transform = Transform::from_xyz(0.0, 1000.0, 0.0)
        .with_rotation(Quat::from_rotation_y(PI) * Quat::from_rotation_x(trim_alpha));

    commands
        .spawn((
            default_aircraft(),
            ControlInput::default(),
            Mesh3d(root_mesh),
            MeshMaterial3d(stealth_dark.clone()),
            start_transform,
        ))
        .with_children(|parent| {
            // ── CENTER BODY (blended wing-body) ──────────────────────
            // The B-2 center body is thicker at the front and tapers aft.

            // Forward center body — thicker nose section
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.0, 2.4, 4.0))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(0.0, 0.1, -10.0),
            ));

            // Nose tip — pointed, angular
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.0, 1.6, 2.5))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(0.0, 0.0, -12.8),
            ));

            // Extreme nose point
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.2, 0.8, 1.5))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(0.0, -0.1, -14.2),
            ));

            // Upper center body fairing — smooth dorsal hump
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.5, 0.8, 10.0))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(0.0, 1.3, -3.0),
            ));

            // Lower center body — flat underside
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.5, 0.4, 14.0))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(0.0, -1.1, -1.0),
            ));

            // ── COCKPIT WINDSHIELD ───────────────────────────────────
            // Small bubble at the front center, two side-by-side windows.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.8, 0.6, 2.0))),
                MeshMaterial3d(cockpit_mat.clone()),
                Transform::from_xyz(0.0, 1.5, -10.5),
            ));

            // Cockpit frame — dark surround
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.2, 0.15, 2.4))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(0.0, 1.85, -10.5),
            ));

            // Cockpit side frames
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.15, 0.6, 2.0))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-1.5, 1.5, -10.5),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.15, 0.6, 2.0))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(1.5, 1.5, -10.5),
            ));

            // Center divider between the two windshield panes
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 0.5, 1.8))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(0.0, 1.5, -10.5),
            ));

            // ── MAIN WINGS — LEFT AND RIGHT ─────────────────────────
            // The B-2 has a continuous flying wing. Each outer wing panel
            // extends from the center body (~4 m from centerline) to
            // the wingtip (~26 m from centerline).
            // Leading edge sweeps back at ~33 degrees.

            // -- Left inner wing panel (4 m to 14 m from center) --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 1.2, 10.0))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(-9.0, 0.0, -1.0),
            ));

            // Left inner wing — upper skin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.3, 9.5))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(-9.0, 0.65, -1.0),
            ));

            // -- Left outer wing panel (14 m to 26 m from center) --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.7, 6.5))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(-20.0, 0.0, 0.5),
            ));

            // Left outer wing — upper skin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.2, 6.0))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(-20.0, 0.40, 0.5),
            ));

            // Left wingtip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.35, 3.0))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(-26.5, 0.0, 1.5),
            ));

            // -- Right inner wing panel --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 1.2, 10.0))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(9.0, 0.0, -1.0),
            ));

            // Right inner wing — upper skin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.3, 9.5))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(9.0, 0.65, -1.0),
            ));

            // -- Right outer wing panel --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.7, 6.5))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(20.0, 0.0, 0.5),
            ));

            // Right outer wing — upper skin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.2, 6.0))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(20.0, 0.40, 0.5),
            ));

            // Right wingtip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.35, 3.0))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(26.5, 0.0, 1.5),
            ));

            // ── LEADING EDGE — swept back, angular ──────────────────
            // Thin strips along the front of each wing to sharpen the edge.

            // Left leading edge — inner
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.3, 0.4))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-9.0, -0.1, -6.2),
            ));

            // Left leading edge — outer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.2, 0.3))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-20.0, -0.05, -2.5),
            ));

            // Right leading edge — inner
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.3, 0.4))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(9.0, -0.1, -6.2),
            ));

            // Right leading edge — outer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.2, 0.3))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(20.0, -0.05, -2.5),
            ));

            // ── W-SHAPED TRAILING EDGE (sawtooth pattern) ───────────
            // The B-2's distinctive trailing edge has a W / sawtooth shape.
            // We model this with angled trailing-edge segments.

            // Left inner trailing edge — angled inward
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(9.0, 0.25, 0.4))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-8.5, -0.1, 4.2)
                    .with_rotation(Quat::from_rotation_y(0.25)),
            ));

            // Left outer trailing edge — angled outward
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.18, 0.35))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-19.5, -0.05, 4.0)
                    .with_rotation(Quat::from_rotation_y(-0.20)),
            ));

            // Right inner trailing edge — angled inward
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(9.0, 0.25, 0.4))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(8.5, -0.1, 4.2)
                    .with_rotation(Quat::from_rotation_y(-0.25)),
            ));

            // Right outer trailing edge — angled outward
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(12.0, 0.18, 0.35))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(19.5, -0.05, 4.0)
                    .with_rotation(Quat::from_rotation_y(0.20)),
            ));

            // Center trailing edge notch (the center V of the W)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.0, 0.3, 0.4))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-2.5, -0.1, 8.5)
                    .with_rotation(Quat::from_rotation_y(-0.30)),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.0, 0.3, 0.4))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(2.5, -0.1, 8.5)
                    .with_rotation(Quat::from_rotation_y(0.30)),
            ));

            // ── ENGINE INTAKES (top of wing, two per side) ──────────
            // The four GE F118 engines are buried in the wing with
            // top-mounted intakes to reduce radar signature.

            // Left inner intake
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.8, 0.6, 3.0))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(-4.5, 1.1, -3.0),
            ));
            // Left inner intake lip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.15, 0.4))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(-4.5, 1.45, -4.5),
            ));

            // Left outer intake
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.8, 0.6, 3.0))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(-7.5, 1.0, -2.5),
            ));
            // Left outer intake lip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.15, 0.4))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(-7.5, 1.35, -4.0),
            ));

            // Right inner intake
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.8, 0.6, 3.0))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(4.5, 1.1, -3.0),
            ));
            // Right inner intake lip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.15, 0.4))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(4.5, 1.45, -4.5),
            ));

            // Right outer intake
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.8, 0.6, 3.0))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(7.5, 1.0, -2.5),
            ));
            // Right outer intake lip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.15, 0.4))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(7.5, 1.35, -4.0),
            ));

            // ── ENGINE EXHAUST (top-rear, blended into trailing edge) ─
            // Flat, wide exhaust slots to minimize IR signature.

            // Left inner exhaust
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.3, 1.5))),
                MeshMaterial3d(exhaust_mat.clone()),
                Transform::from_xyz(-4.5, 0.6, 5.5),
            ));

            // Left outer exhaust
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.3, 1.5))),
                MeshMaterial3d(exhaust_mat.clone()),
                Transform::from_xyz(-7.5, 0.5, 5.0),
            ));

            // Right inner exhaust
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.3, 1.5))),
                MeshMaterial3d(exhaust_mat.clone()),
                Transform::from_xyz(4.5, 0.6, 5.5),
            ));

            // Right outer exhaust
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.3, 1.5))),
                MeshMaterial3d(exhaust_mat.clone()),
                Transform::from_xyz(7.5, 0.5, 5.0),
            ));

            // ── SPLIT AILERON / YAW CONTROL SURFACES ────────────────
            // The B-2 uses split drag-rudders at the wingtips for yaw
            // control instead of a vertical tail.

            // Left split aileron (upper)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.5, 0.12, 1.5))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(-24.0, 0.25, 2.5),
            ));

            // Left split aileron (lower)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.5, 0.12, 1.5))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(-24.0, -0.25, 2.5),
            ));

            // Right split aileron (upper)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.5, 0.12, 1.5))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(24.0, 0.25, 2.5),
            ));

            // Right split aileron (lower)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.5, 0.12, 1.5))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(24.0, -0.25, 2.5),
            ));

            // ── ELEVONS (trailing-edge control surfaces) ────────────
            // Multiple elevon segments along the trailing edge.

            // Left inner elevon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.0, 0.15, 1.2))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(-7.0, -0.1, 5.0),
            ));

            // Left outer elevon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(8.0, 0.12, 1.0))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(-18.0, -0.05, 4.2),
            ));

            // Right inner elevon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.0, 0.15, 1.2))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(7.0, -0.1, 5.0),
            ));

            // Right outer elevon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(8.0, 0.12, 1.0))),
                MeshMaterial3d(stealth_light.clone()),
                Transform::from_xyz(18.0, -0.05, 4.2),
            ));

            // ── TRICYCLE LANDING GEAR ────────────────────────────────
            // Nosewheel forward under center body, two main gear under
            // the inner wing sections. Gear is tucked / short since the
            // B-2 sits very low.

            let wheel_mesh = meshes.add(Cylinder::new(0.45, 0.25));

            // -- Nosewheel strut --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.15, 1.5, 0.15))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(0.0, -1.8, -9.0),
            ));

            // Nosewheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(0.0, -2.6, -9.0)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // -- Left main gear strut --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 1.8, 0.18))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-4.0, -1.9, 0.0),
            ));

            // Left main gear bogie (dual wheels)
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-4.0, -2.9, -0.5)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-4.0, -2.9, 0.5)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // -- Right main gear strut --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 1.8, 0.18))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(4.0, -1.9, 0.0),
            ));

            // Right main gear bogie (dual wheels)
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(4.0, -2.9, -0.5)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(4.0, -2.9, 0.5)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // ── GEAR DOORS (closed position, flush with underside) ──
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.5, 0.08, 2.5))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(-4.0, -1.15, 0.0),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.5, 0.08, 2.5))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(4.0, -1.15, 0.0),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.8, 0.08, 1.8))),
                MeshMaterial3d(stealth_dark.clone()),
                Transform::from_xyz(0.0, -1.15, -9.0),
            ));

            // ── PANEL LINES / SURFACE DETAIL ────────────────────────
            // Subtle panel line accents on the upper surface to break
            // up the flat appearance.

            // Centerline panel seam
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.04, 0.02, 18.0))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(0.0, 1.02, -2.0),
            ));

            // Left wing panel seams
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, 0.02, 8.0))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-6.0, 0.82, -1.0),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, 0.02, 6.0))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(-14.0, 0.52, 0.0),
            ));

            // Right wing panel seams
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, 0.02, 8.0))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(6.0, 0.82, -1.0),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.03, 0.02, 6.0))),
                MeshMaterial3d(edge_mat.clone()),
                Transform::from_xyz(14.0, 0.52, 0.0),
            ));

            // ── WEAPON BAY DOORS (two side-by-side bays) ────────────
            // Visible as seam lines on the underside of the center body.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.5, 0.05, 5.0))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(-1.8, -1.32, -1.0),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.5, 0.05, 5.0))),
                MeshMaterial3d(stealth_mid.clone()),
                Transform::from_xyz(1.8, -1.32, -1.0),
            ));
        });
}
