use bevy::prelude::*;
use std::f32::consts::PI;

use super::{AileronLeft, AileronRight, Aircraft, ControlInput, Elevator, Rudder};

/// Boeing 737-style airliner specifications.
pub fn default_aircraft() -> Aircraft {
    Aircraft {
        velocity: Vec3::new(0.0, 0.0, 230.0),
        throttle: 0.70,
        angular_velocity: Vec3::ZERO,
        mass: 62000.0,
        wing_area: 124.6,
        max_thrust: 240000.0,
        cd0: 0.020,
        oswald_efficiency: 0.78,
        aspect_ratio: 9.45,
        pitch_rate: 20.0_f32.to_radians(),
        roll_rate: 30.0_f32.to_radians(),
        yaw_rate: 15.0_f32.to_radians(),
        side_force_coeff: 1.5,
        alpha: 0.0,
        g_load: 1.0,
    }
}

/// Spawn the airliner at altitude 1000m heading north.
///
/// The mesh is modeled with the nose at -Z (Bevy's forward convention).
/// Nose faces -Z, tail faces +Z.
///
/// Realistic Boeing 737-800 proportions:
///   Wingspan ~34m, fuselage length ~38m, fuselage diameter ~3.8m
///   LOW WING with swept planform (~25 deg), twin underwing engines,
///   conventional tail with dark blue livery on belly and tail.
///   White upper fuselage, dark blue lower fuselage.
pub fn spawn_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Materials ──────────────────────────────────────────────────────
    let white_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.96, 0.96, 0.96),
        ..default()
    });
    let dark_blue_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.20, 0.45),
        ..default()
    });
    let wing_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.88),
        ..default()
    });
    let dark_grey_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.25, 0.28),
        ..default()
    });
    let windshield_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.08, 0.10, 0.18, 0.60),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let engine_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.55, 0.58),
        ..default()
    });
    let engine_intake_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.18, 0.20),
        ..default()
    });
    let radome_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.40, 0.40, 0.42),
        ..default()
    });

    // ── Fuselage reference frame ───────────────────────────────────────
    // Total fuselage: ~38m along Z.  Nose at roughly Z = -19, tail at Z = +19.
    // Root entity placed at approximate CG (~25% MAC), which sits near Z = -2.
    // Fuselage diameter ~3.8m. Y = 0 is fuselage centerline.
    // Wings are LOW — wing top surface at Y ~ -0.6 (below centerline).

    // Root cuboid: main cabin upper half (white)
    let root_mesh = meshes.add(Cuboid::new(3.80, 2.00, 10.0));

    // Trim angle of attack at cruise: alpha ~ 0.020 rad
    let trim_alpha: f32 = 0.020;
    let start_transform = Transform::from_xyz(0.0, 1000.0, 0.0)
        .with_rotation(Quat::from_rotation_y(PI) * Quat::from_rotation_x(trim_alpha));

    commands
        .spawn((
            default_aircraft(),
            ControlInput::default(),
            Mesh3d(root_mesh),
            MeshMaterial3d(white_mat.clone()),
            start_transform,
        ))
        .with_children(|parent| {
            // ── FUSELAGE — UPPER (white) ─────────────────────────────────
            // Root covers Z = -5..+5 upper half. Additional sections taper
            // toward nose and tail. Upper half: Y = 0..+1.90

            // Forward cabin upper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.70, 1.95, 4.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.025, -7.0),
            ));

            // Forward taper 1 upper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.40, 1.80, 3.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.10, -10.5),
            ));

            // Nose taper 2 upper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.00, 1.60, 2.5))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.15, -13.25),
            ));

            // Nose cone upper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.50, 1.30, 2.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.20, -15.5),
            ));

            // Nose tip / radome
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 1.00, 1.5))),
                MeshMaterial3d(radome_mat.clone()),
                Transform::from_xyz(0.0, 0.15, -17.25),
            ));

            // Radome cap
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.00, 0.70, 0.8))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.0, 0.10, -18.40),
            ));

            // Aft cabin upper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.70, 1.95, 4.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.025, 7.0),
            ));

            // Aft taper 1 upper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.30, 1.70, 3.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.15, 10.5),
            ));

            // Aft taper 2 upper
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 1.40, 2.5))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.25, 13.25),
            ));

            // ── FUSELAGE — LOWER (dark blue belly) ───────────────────────
            // Lower half mirrors the upper sections but in dark blue.
            // Lower half: Y = -1.90..0

            // Main cabin lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.80, 1.80, 10.0))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.90, 0.0),
            ));

            // Forward cabin lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.70, 1.75, 4.0))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.875, -7.0),
            ));

            // Forward taper 1 lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.40, 1.60, 3.0))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.75, -10.5),
            ));

            // Nose taper 2 lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.00, 1.30, 2.5))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.55, -13.25),
            ));

            // Nose cone lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.50, 1.00, 2.0))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.35, -15.5),
            ));

            // Aft cabin lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.70, 1.75, 4.0))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.875, 7.0),
            ));

            // Aft taper 1 lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.30, 1.50, 3.0))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.65, 10.5),
            ));

            // Aft taper 2 lower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 1.10, 2.5))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -0.40, 13.25),
            ));

            // Tail cone (dark blue, tapers to APU)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 1.80, 2.0))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, 0.30, 15.5),
            ));

            // Tail tip — APU exhaust area
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.00, 1.00, 1.5))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, 0.40, 17.25),
            ));

            // ── COCKPIT WINDSHIELD ───────────────────────────────────────
            // Angled front windshield — tilted forward to match real 737 profile
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.20, 1.20, 0.06))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(0.0, 0.70, -16.80).with_rotation(Quat::from_rotation_x(-0.40)),
            ));

            // Left cockpit side window
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.05, 0.70, 1.00))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(-1.40, 0.70, -15.8),
            ));

            // Right cockpit side window
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.05, 0.70, 1.00))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(1.40, 0.70, -15.8),
            ));

            // ── BELLY FAIRING ────────────────────────────────────────────
            // The bulge under the wing-body junction for landing gear bay
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.20, 0.60, 6.00))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, -2.10, -0.5),
            ));

            // ── LOW WINGS (swept ~25 deg) ────────────────────────────────
            let wing_y = -0.60;
            let wing_root_z = -1.0;

            // Left wing — inner panel (root to ~7m outboard)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.38, 4.20))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-5.40, wing_y, wing_root_z + 1.5),
            ));

            // Left wing — outer panel (7m to ~16m outboard, swept back)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.50, 0.28, 3.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-12.15, wing_y + 0.30, wing_root_z + 3.2),
            ));

            // Left wing tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.40, 0.20, 2.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-16.10, wing_y + 0.50, wing_root_z + 4.2),
            ));

            // Left winglet — upward-angled piece at wingtip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 1.60, 1.20))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(-16.80, wing_y + 1.50, wing_root_z + 4.0)
                    .with_rotation(Quat::from_rotation_z(0.15)),
            ));

            // Right wing — inner panel
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.38, 4.20))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(5.40, wing_y, wing_root_z + 1.5),
            ));

            // Right wing — outer panel
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(6.50, 0.28, 3.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(12.15, wing_y + 0.30, wing_root_z + 3.2),
            ));

            // Right wing tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.40, 0.20, 2.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(16.10, wing_y + 0.50, wing_root_z + 4.2),
            ));

            // Right winglet — upward-angled piece at wingtip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 1.60, 1.20))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(16.80, wing_y + 1.50, wing_root_z + 4.0)
                    .with_rotation(Quat::from_rotation_z(-0.15)),
            ));

            // Wing root fairing — blends wing into fuselage belly
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.00, 0.50, 5.00))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, wing_y + 0.05, wing_root_z + 1.0),
            ));

            // ── ENGINE NACELLES (underwing, on pylons) ───────────────────
            let engine_outboard = 5.80;
            let engine_y = wing_y - 1.60;
            let engine_z = wing_root_z - 0.5;

            // -- Left engine --
            // Main nacelle body
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.70, 1.70, 4.2))),
                MeshMaterial3d(engine_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z),
            ));

            // Intake ring — larger diameter, visible lip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.00, 2.00, 0.35))),
                MeshMaterial3d(engine_intake_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z - 2.30),
            ));

            // Intake face (dark interior)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.60, 1.60, 0.10))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z - 2.55),
            ));

            // Exhaust cone (tapered rear)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.30, 1.30, 0.80))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z + 2.50),
            ));

            // Left pylon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.28, 1.20, 2.50))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y + 1.40, engine_z + 0.5),
            ));

            // -- Right engine --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.70, 1.70, 4.2))),
                MeshMaterial3d(engine_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z),
            ));

            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.00, 2.00, 0.35))),
                MeshMaterial3d(engine_intake_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z - 2.30),
            ));

            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.60, 1.60, 0.10))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z - 2.55),
            ));

            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.30, 1.30, 0.80))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z + 2.50),
            ));

            // Right pylon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.28, 1.20, 2.50))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y + 1.40, engine_z + 0.5),
            ));

            // ── VERTICAL STABILIZER (dark blue tail) ─────────────────────
            // Main fin — dark blue livery
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.20, 5.50, 4.50))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, 3.80, 15.0),
            ));

            // Upper fin tip — narrower, dark blue
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 1.80, 2.50))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, 7.40, 14.0),
            ));

            // Dorsal fin fillet — blends fin into fuselage
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.16, 1.00, 2.00))),
                MeshMaterial3d(dark_blue_mat.clone()),
                Transform::from_xyz(0.0, 1.50, 13.5),
            ));

            // ── HORIZONTAL STABILIZER ────────────────────────────────────
            // Left horizontal stab
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(5.50, 0.18, 3.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-3.50, 1.20, 16.5),
            ));

            // Right horizontal stab
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(5.50, 0.18, 3.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(3.50, 1.20, 16.5),
            ));

            // Horizontal stab root fairing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.00, 0.20, 3.20))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(0.0, 1.20, 16.5),
            ));

            // ── CONTROL SURFACES ─────────────────────────────────────────
            let ctrl_surface_mat = materials.add(StandardMaterial {
                base_color: Color::srgb(0.75, 0.75, 0.78),
                ..default()
            });

            // Left aileron — trailing edge of left outer wing
            parent.spawn((
                AileronLeft,
                Mesh3d(meshes.add(Cuboid::new(5.0, 0.08, 0.8))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(-12.15, wing_y + 0.30, wing_root_z + 4.8),
            ));

            // Right aileron — trailing edge of right outer wing
            parent.spawn((
                AileronRight,
                Mesh3d(meshes.add(Cuboid::new(5.0, 0.08, 0.8))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(12.15, wing_y + 0.30, wing_root_z + 4.8),
            ));

            // Left elevator — trailing edge of left horizontal stabilizer
            parent.spawn((
                Elevator,
                Mesh3d(meshes.add(Cuboid::new(4.5, 0.07, 0.7))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(-3.50, 1.20, 18.0),
            ));

            // Right elevator — trailing edge of right horizontal stabilizer
            parent.spawn((
                Elevator,
                Mesh3d(meshes.add(Cuboid::new(4.5, 0.07, 0.7))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(3.50, 1.20, 18.0),
            ));

            // Rudder — trailing edge of vertical stabilizer
            parent.spawn((
                Rudder {
                    base_rotation: Quat::IDENTITY,
                },
                Mesh3d(meshes.add(Cuboid::new(0.08, 4.0, 0.8))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(0.0, 3.80, 17.30),
            ));
        });
}
