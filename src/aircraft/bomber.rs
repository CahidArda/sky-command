use bevy::prelude::*;
use std::f32::consts::PI;

use super::{Aircraft, ControlInput, AileronLeft, AileronRight, Elevator};

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
///   Pure flying wing — no conventional fuselage or vertical tail.
///   Smooth blended center body flowing into swept wings (~33 deg).
///   W-shaped trailing edge, raised dorsal hump for crew/intakes,
///   cockpit windshield at front tip, dark charcoal stealth finish.
///
/// ~42 entities total (1 root + 41 children).
pub fn spawn_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Materials ──────────────────────────────────────────────────────
    // Base charcoal — almost black
    let dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.17),
        ..default()
    });
    // Slightly lighter underside
    let under = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.18, 0.20),
        ..default()
    });
    // Dorsal hump — subtly darker than base
    let hump_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.12, 0.14),
        ..default()
    });
    // Exhaust slots — near black
    let exhaust_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.07, 0.07, 0.07),
        ..default()
    });
    // Cockpit glass
    let cockpit_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.20, 0.28, 0.35, 0.55),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    // Control surfaces — slightly lighter for visibility
    let ctrl_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.25, 0.27),
        ..default()
    });

    // Wing sweep angle: ~33 degrees
    let sweep: f32 = 33.0_f32.to_radians();

    // ── Root entity ──────────────────────────────────────────────────
    // Central body core — the widest, thickest part of the blended body.
    // 10 m wide, 1.8 m tall, 14 m long (chord).
    let root_mesh = meshes.add(Cuboid::new(10.0, 1.8, 14.0));

    let trim_alpha: f32 = 0.004;
    let start_transform = Transform::from_xyz(0.0, 1000.0, 0.0)
        .with_rotation(Quat::from_rotation_y(PI) * Quat::from_rotation_x(trim_alpha));

    commands
        .spawn((
            default_aircraft(),
            ControlInput::default(),
            Mesh3d(root_mesh),
            MeshMaterial3d(dark.clone()),
            start_transform,
        ))
        .with_children(|parent| {
            // ── CENTER BODY — blended taper toward nose ──────────────
            // Overlapping cuboids of decreasing width/height create a
            // smooth taper from the center body into the nose.

            // Forward taper — narrower, slightly thinner
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.0, 1.5, 4.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(0.0, 0.0, -9.0),
            ));

            // Nose section — angular, flat
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.0, 1.0, 3.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(0.0, -0.05, -11.8),
            ));

            // Nose tip — very narrow point
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.6, 0.5, 2.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(0.0, -0.15, -14.0),
            ));

            // ── COCKPIT WINDSHIELD ─────────────────────────────────
            // Small, dark glass at the very front tip of the center body.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 0.4, 1.6))),
                MeshMaterial3d(cockpit_mat.clone()),
                Transform::from_xyz(0.0, 0.55, -11.5),
            ));

            // ── DORSAL HUMP — crew compartment / engine intakes ────
            // A raised section on top of the center body, slightly
            // darker. This houses the two-person crew and the four
            // top-mounted engine intakes.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(8.0, 0.9, 8.0))),
                MeshMaterial3d(hump_mat.clone()),
                Transform::from_xyz(0.0, 1.2, -2.0),
            ));

            // Forward hump taper — blends into the nose section
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(5.5, 0.6, 3.0))),
                MeshMaterial3d(hump_mat.clone()),
                Transform::from_xyz(0.0, 1.05, -7.5),
            ));

            // ── FLAT UNDERSIDE ─────────────────────────────────────
            // Slightly lighter panel on the bottom — smooth belly.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(9.0, 0.3, 13.0))),
                MeshMaterial3d(under.clone()),
                Transform::from_xyz(0.0, -1.0, -0.5),
            ));

            // ── WING — LEFT SIDE ───────────────────────────────────
            // Three overlapping panels of decreasing chord and thickness
            // create the blended wing taper. Each is rotated to match
            // the ~33 degree leading-edge sweep.

            // Left inner wing (blend zone, 4-12 m from center)
            // Chord tapers from ~12 m to ~9 m, thickness 1.2 m to 0.8 m.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(9.0, 1.1, 11.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-8.5, -0.05, 0.0)
                    .with_rotation(Quat::from_rotation_y(sweep * 0.15)),
            ));

            // Left mid wing (12-20 m from center)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(9.0, 0.65, 7.5))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-16.5, -0.05, 1.5)
                    .with_rotation(Quat::from_rotation_y(sweep * 0.12)),
            ));

            // Left outer wing (20-26 m from center) — thinnest
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.0, 0.35, 4.5))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-23.5, -0.02, 2.5)
                    .with_rotation(Quat::from_rotation_y(sweep * 0.10)),
            ));

            // Left wingtip — tapered to a thin edge
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.8, 0.18, 2.5))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-26.8, 0.0, 3.2),
            ));

            // ── WING — RIGHT SIDE (mirror of left) ────────────────
            // Right inner wing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(9.0, 1.1, 11.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(8.5, -0.05, 0.0)
                    .with_rotation(Quat::from_rotation_y(-sweep * 0.15)),
            ));

            // Right mid wing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(9.0, 0.65, 7.5))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(16.5, -0.05, 1.5)
                    .with_rotation(Quat::from_rotation_y(-sweep * 0.12)),
            ));

            // Right outer wing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.0, 0.35, 4.5))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(23.5, -0.02, 2.5)
                    .with_rotation(Quat::from_rotation_y(-sweep * 0.10)),
            ));

            // Right wingtip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.8, 0.18, 2.5))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(26.8, 0.0, 3.2),
            ));

            // ── WING-BODY BLEND — fills the junction ──────────────
            // Smooth transition pieces that fill the gap between the
            // thick center body and the thinner inner wing panels.
            // Left blend
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.0, 1.4, 12.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-6.0, -0.05, -0.5),
            ));

            // Right blend
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.0, 1.4, 12.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(6.0, -0.05, -0.5),
            ));

            // ── LEADING EDGE — swept, sharp ──────────────────────
            // Rotated thin strips along the front of each wing to
            // create the distinctive swept leading edge.

            // Left leading edge — full span
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(22.0, 0.25, 0.35))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-15.0, -0.05, -4.0)
                    .with_rotation(Quat::from_rotation_y(sweep * 0.45)),
            ));

            // Right leading edge — full span
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(22.0, 0.25, 0.35))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(15.0, -0.05, -4.0)
                    .with_rotation(Quat::from_rotation_y(-sweep * 0.45)),
            ));

            // ── W-SHAPED TRAILING EDGE ──────────────────────────
            // The B-2's signature sawtooth / W trailing edge.
            // Six segments: center-left V, center-right V,
            // left inner (angled inboard), left outer (angled outboard),
            // right inner (angled inboard), right outer (angled outboard).

            // Center V — left arm (angled from center toward ~5 m left)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(5.5, 0.22, 0.35))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-3.0, -0.1, 7.8)
                    .with_rotation(Quat::from_rotation_y(-0.35)),
            ));

            // Center V — right arm
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(5.5, 0.22, 0.35))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(3.0, -0.1, 7.8)
                    .with_rotation(Quat::from_rotation_y(0.35)),
            ));

            // Left inner trailing edge — sweeps aft-inboard
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.20, 0.30))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-11.0, -0.08, 5.5)
                    .with_rotation(Quat::from_rotation_y(0.22)),
            ));

            // Left outer trailing edge — sweeps aft-outboard
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.15, 0.28))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(-21.0, -0.04, 4.8)
                    .with_rotation(Quat::from_rotation_y(-0.18)),
            ));

            // Right inner trailing edge — sweeps aft-inboard
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.20, 0.30))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(11.0, -0.08, 5.5)
                    .with_rotation(Quat::from_rotation_y(-0.22)),
            ));

            // Right outer trailing edge — sweeps aft-outboard
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(10.0, 0.15, 0.28))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(21.0, -0.04, 4.8)
                    .with_rotation(Quat::from_rotation_y(0.18)),
            ));

            // ── EXHAUST — flat slots at trailing edge of center ───
            // Two wide, flat exhaust slots blended into the aft body.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(5.0, 0.20, 1.2))),
                MeshMaterial3d(exhaust_mat.clone()),
                Transform::from_xyz(-3.5, 0.4, 6.5),
            ));

            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(5.0, 0.20, 1.2))),
                MeshMaterial3d(exhaust_mat.clone()),
                Transform::from_xyz(3.5, 0.4, 6.5),
            ));

            // ── WING UNDERSIDE PANELS ─────────────────────────────
            // Lighter-colored belly panels on the inner wings.
            // Left
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(8.0, 0.15, 9.0))),
                MeshMaterial3d(under.clone()),
                Transform::from_xyz(-8.5, -0.65, 0.0),
            ));

            // Right
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(8.0, 0.15, 9.0))),
                MeshMaterial3d(under.clone()),
                Transform::from_xyz(8.5, -0.65, 0.0),
            ));

            // ── AFT CENTER BODY TAPER ─────────────────────────────
            // The center body narrows slightly aft of the exhaust
            // before meeting the center-V trailing edge.
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.0, 1.0, 3.0))),
                MeshMaterial3d(dark.clone()),
                Transform::from_xyz(0.0, 0.0, 6.0),
            ));

            // ── CONTROL SURFACES (elevons — no rudder on B-2) ─────
            // The B-2 uses elevons: ailerons on outer wings for roll,
            // elevators on inner wings for pitch. No vertical tail,
            // so no rudder marker.

            // Left aileron — outer wing trailing edge
            parent.spawn((
                AileronLeft,
                Mesh3d(meshes.add(Cuboid::new(8.0, 0.08, 1.0))),
                MeshMaterial3d(ctrl_mat.clone()),
                Transform::from_xyz(-20.0, -0.06, 5.0),
            ));

            // Right aileron — outer wing trailing edge
            parent.spawn((
                AileronRight,
                Mesh3d(meshes.add(Cuboid::new(8.0, 0.08, 1.0))),
                MeshMaterial3d(ctrl_mat.clone()),
                Transform::from_xyz(20.0, -0.06, 5.0),
            ));

            // Left elevator — inner wing trailing edge
            parent.spawn((
                Elevator,
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.10, 1.0))),
                MeshMaterial3d(ctrl_mat.clone()),
                Transform::from_xyz(-7.5, -0.06, 6.0),
            ));

            // Right elevator — inner wing trailing edge
            parent.spawn((
                Elevator,
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.10, 1.0))),
                MeshMaterial3d(ctrl_mat.clone()),
                Transform::from_xyz(7.5, -0.06, 6.0),
            ));
        });
}
