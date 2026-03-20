use bevy::prelude::*;
use std::f32::consts::PI;

use super::{AileronLeft, AileronRight, Aircraft, ControlInput, Elevator, Rudder};

/// F-15 Eagle specifications.
pub fn default_aircraft() -> Aircraft {
    Aircraft {
        velocity: Vec3::new(0.0, 0.0, 300.0),
        throttle: 0.50,
        angular_velocity: Vec3::ZERO,
        mass: 20000.0,
        wing_area: 56.5,
        max_thrust: 210000.0,
        cd0: 0.022,
        oswald_efficiency: 0.85,
        aspect_ratio: 3.01,
        pitch_rate: 90.0_f32.to_radians(),
        roll_rate: 240.0_f32.to_radians(),
        yaw_rate: 45.0_f32.to_radians(),
        side_force_coeff: 3.0,
        alpha: 0.0,
        g_load: 1.0,
    }
}

/// Spawn an F-15 Eagle at altitude 1000 m heading north.
///
/// The mesh is modeled with the nose at -Z (Bevy's forward convention).
/// Nose faces -Z, tail faces +Z.
///
/// Simplified F-15 proportions (~50 child entities):
///   Length ~19.4 m, wingspan ~13 m, height ~5.6 m
///   Twin canted vertical stabilizers, twin P&W F100 engines,
///   wide flat fuselage, large rectangular side intakes,
///   bubble canopy, conformal fuel tanks, underwing drop tanks,
///   tricycle landing gear.
///
/// Two-tone grey camouflage: lighter upper surfaces, darker lower/panels.
pub fn spawn_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Materials (reduced set) ──────────────────────────────────────
    let camo_light = materials.add(StandardMaterial {
        base_color: Color::srgb(0.68, 0.69, 0.70),
        ..default()
    });
    let camo_dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.52, 0.53, 0.54),
        ..default()
    });
    let dark_accent = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.36, 0.37),
        ..default()
    });
    let very_dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.18, 0.20),
        ..default()
    });
    let canopy_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.30, 0.45, 0.60, 0.45),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let canopy_frame_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.30, 0.32),
        ..default()
    });
    let intake_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.25, 0.27),
        ..default()
    });
    let nozzle_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.27, 0.25),
        ..default()
    });
    let radome_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.43, 0.45),
        ..default()
    });
    let gear_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.18, 0.20),
        ..default()
    });
    let tire_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.10, 0.10),
        ..default()
    });
    let tank_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.60, 0.61, 0.62),
        ..default()
    });
    let ctrl_surface_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.50, 0.51, 0.52),
        ..default()
    });

    // ── Reference frame ──────────────────────────────────────────────
    // F-15 length ~19.4 m.  Nose at Z ~ -9.7, tail at Z ~ +9.7.
    // CG at Z = 0.  Y = 0 is the fuselage mid-height.
    // Fuselage is wide & flat: ~4.0 m wide at intakes, ~1.4 m tall.

    // Root entity mesh — the central fuselage core section (widest part).
    let root_mesh = meshes.add(Cuboid::new(2.60, 1.40, 5.00));

    let trim_alpha: f32 = 0.010;
    let start_transform = Transform::from_xyz(0.0, 1000.0, 0.0)
        .with_rotation(Quat::from_rotation_y(PI) * Quat::from_rotation_x(trim_alpha));

    commands
        .spawn((
            default_aircraft(),
            ControlInput::default(),
            Mesh3d(root_mesh),
            MeshMaterial3d(camo_light.clone()),
            start_transform,
        ))
        .with_children(|parent| {
            // ══════════════════════════════════════════════════════════
            // ── FUSELAGE (6 sections, nose to tail)
            // ══════════════════════════════════════════════════════════

            // 1. Forward fuselage — tapering toward nose
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.20, 1.30, 2.80))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(0.0, 0.0, -3.90),
            ));

            // 2. Forward fuselage — further tapering (behind cockpit)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.70, 1.15, 2.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(0.0, -0.02, -6.20),
            ));

            // 3. Nose section
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.20, 0.90, 1.60))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(0.0, -0.05, -7.90),
            ));

            // 4. Radome (merged: tip + taper into one piece)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.55, 0.45, 1.80))),
                MeshMaterial3d(radome_mat.clone()),
                Transform::from_xyz(0.0, -0.05, -9.30),
            ));

            // 5. Aft fuselage — between wings and engine nacelles
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 1.35, 3.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(0.0, 0.0, 4.10),
            ));

            // 6. Tail section — narrows as fuselage splits into twin nacelles
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.20, 1.20, 2.80))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(0.0, 0.0, 6.90),
            ));

            // Engine fairing — spine between the two nacelles
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.70, 0.60, 2.20))),
                MeshMaterial3d(dark_accent.clone()),
                Transform::from_xyz(0.0, 0.10, 8.60),
            ));

            // ══════════════════════════════════════════════════════════
            // ── DORSAL SPINE (single merged piece)
            // ══════════════════════════════════════════════════════════
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.45, 0.12, 9.00))),
                MeshMaterial3d(dark_accent.clone()),
                Transform::from_xyz(0.0, 0.76, -1.00),
            ));

            // ══════════════════════════════════════════════════════════
            // ── BUBBLE CANOPY
            // ══════════════════════════════════════════════════════════

            // Canopy sill / base frame
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.95, 0.15, 2.50))),
                MeshMaterial3d(canopy_frame_mat.clone()),
                Transform::from_xyz(0.0, 0.73, -5.60),
            ));

            // Canopy glass — main bubble + dome merged
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 0.60, 2.20))),
                MeshMaterial3d(canopy_mat.clone()),
                Transform::from_xyz(0.0, 1.10, -5.60),
            ));

            // Windscreen — front angled glass panel
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 0.45, 0.04))),
                MeshMaterial3d(canopy_mat.clone()),
                Transform::from_xyz(0.0, 0.95, -6.90).with_rotation(Quat::from_rotation_x(-0.45)),
            ));

            // ══════════════════════════════════════════════════════════
            // ── SIDE INTAKES (2 pieces per side: body + mouth)
            // ══════════════════════════════════════════════════════════

            // Left intake body
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.90, 1.10, 3.80))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(-1.75, -0.10, -2.60),
            ));

            // Left intake opening
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 0.95, 0.30))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(-1.75, -0.15, -4.45),
            ));

            // Right intake body
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.90, 1.10, 3.80))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(1.75, -0.10, -2.60),
            ));

            // Right intake opening
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 0.95, 0.30))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(1.75, -0.15, -4.45),
            ));

            // ══════════════════════════════════════════════════════════
            // ── CONFORMAL FUEL TANKS (1 piece per side)
            // ══════════════════════════════════════════════════════════

            // Left CFT
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.45, 0.70, 3.50))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(-1.52, -0.05, 0.80),
            ));

            // Right CFT
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.45, 0.70, 3.50))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(1.52, -0.05, 0.80),
            ));

            // ══════════════════════════════════════════════════════════
            // ── WINGS (2 sections per side: inner + outer)
            // ══════════════════════════════════════════════════════════

            let wing_y = -0.18;
            let wing_z = 0.60;

            // Left wing — inner (root to mid)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.80, 0.14, 4.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(-2.70, wing_y, wing_z),
            ));

            // Left wing — outer (mid to tip)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 0.09, 2.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(-5.30, wing_y, wing_z + 1.00),
            ));

            // Right wing — inner
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.80, 0.14, 4.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(2.70, wing_y, wing_z),
            ));

            // Right wing — outer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 0.09, 2.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(5.30, wing_y, wing_z + 1.00),
            ));

            // ══════════════════════════════════════════════════════════
            // ── TWIN CANTED VERTICAL STABILIZERS (~15° outward)
            // ══════════════════════════════════════════════════════════

            let vtail_z = 7.20;
            let vtail_cant = 15.0_f32.to_radians();

            // Left vertical stabilizer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 3.50, 2.80))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(-1.40, 2.40, vtail_z)
                    .with_rotation(Quat::from_rotation_z(vtail_cant)),
            ));

            // Right vertical stabilizer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 3.50, 2.80))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(1.40, 2.40, vtail_z)
                    .with_rotation(Quat::from_rotation_z(-vtail_cant)),
            ));

            // ══════════════════════════════════════════════════════════
            // ── HORIZONTAL STABILIZERS (1 piece per side)
            // ══════════════════════════════════════════════════════════

            let htail_z = 8.20;
            let htail_y = 0.05;

            // Left horizontal stabilizer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.80, 0.08, 2.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(-2.10, htail_y, htail_z),
            ));

            // Right horizontal stabilizer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.80, 0.08, 2.20))),
                MeshMaterial3d(camo_light.clone()),
                Transform::from_xyz(2.10, htail_y, htail_z),
            ));

            // ══════════════════════════════════════════════════════════
            // ── TWIN ENGINE NACELLES (1 piece each)
            // ══════════════════════════════════════════════════════════

            let nacelle_y = -0.05;

            // Left engine nacelle
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.10, 1.15, 4.50))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(-0.72, nacelle_y, 5.75),
            ));

            // Right engine nacelle
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.10, 1.15, 4.50))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(0.72, nacelle_y, 5.75),
            ));

            // ══════════════════════════════════════════════════════════
            // ── ENGINE NOZZLES (1 piece each)
            // ══════════════════════════════════════════════════════════

            let nozzle_z = 9.50;

            // Left nozzle
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.46, 0.80))),
                MeshMaterial3d(nozzle_mat.clone()),
                Transform::from_xyz(-0.72, nacelle_y, nozzle_z)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // Right nozzle
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.46, 0.80))),
                MeshMaterial3d(nozzle_mat.clone()),
                Transform::from_xyz(0.72, nacelle_y, nozzle_z)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // ══════════════════════════════════════════════════════════
            // ── UNDERWING DROP TANKS (pylon + tank per side)
            // ══════════════════════════════════════════════════════════

            let drop_y = wing_y - 0.55;

            // Left pylon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 0.35, 0.80))),
                MeshMaterial3d(dark_accent.clone()),
                Transform::from_xyz(-2.80, wing_y - 0.25, wing_z + 0.20),
            ));

            // Left drop tank
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.18, 3.60))),
                MeshMaterial3d(tank_mat.clone()),
                Transform::from_xyz(-2.80, drop_y, wing_z + 0.20)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // Right pylon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 0.35, 0.80))),
                MeshMaterial3d(dark_accent.clone()),
                Transform::from_xyz(2.80, wing_y - 0.25, wing_z + 0.20),
            ));

            // Right drop tank
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.18, 3.60))),
                MeshMaterial3d(tank_mat.clone()),
                Transform::from_xyz(2.80, drop_y, wing_z + 0.20)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // ══════════════════════════════════════════════════════════
            // ── WINGTIP MISSILE RAILS (1 piece per side)
            // ══════════════════════════════════════════════════════════

            // Left wingtip missile
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.12, 0.12, 1.80))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(-6.20, wing_y, wing_z + 1.30),
            ));

            // Right wingtip missile
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.12, 0.12, 1.80))),
                MeshMaterial3d(camo_dark.clone()),
                Transform::from_xyz(6.20, wing_y, wing_z + 1.30),
            ));

            // ══════════════════════════════════════════════════════════
            // ── TRICYCLE LANDING GEAR
            // ══════════════════════════════════════════════════════════

            let wheel_mesh = meshes.add(Cylinder::new(0.24, 0.16));

            // Nose gear strut
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 1.05, 0.08))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(0.0, -1.22, -6.50),
            ));
            // Nosewheel tire
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(0.0, -1.78, -6.50)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // Left main gear strut
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 1.15, 0.10))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-1.35, -1.28, 0.30),
            ));
            // Left main wheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-1.35, -1.88, 0.30)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // Right main gear strut
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 1.15, 0.10))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(1.35, -1.28, 0.30),
            ));
            // Right main wheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(1.35, -1.88, 0.30)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // ══════════════════════════════════════════════════════════
            // ── CONTROL SURFACES (must keep all markers)
            // ══════════════════════════════════════════════════════════

            // Left aileron
            parent.spawn((
                AileronLeft,
                Mesh3d(meshes.add(Cuboid::new(1.60, 0.06, 0.55))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(-4.60, wing_y, wing_z + 0.70 + 1.65),
            ));

            // Right aileron
            parent.spawn((
                AileronRight,
                Mesh3d(meshes.add(Cuboid::new(1.60, 0.06, 0.55))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(4.60, wing_y, wing_z + 0.70 + 1.65),
            ));

            // Left elevator
            parent.spawn((
                Elevator,
                Mesh3d(meshes.add(Cuboid::new(2.20, 0.05, 0.50))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(-1.80, htail_y, htail_z + 1.35),
            ));

            // Right elevator
            parent.spawn((
                Elevator,
                Mesh3d(meshes.add(Cuboid::new(2.20, 0.05, 0.50))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(1.80, htail_y, htail_z + 1.35),
            ));

            // Left rudder
            let left_cant = Quat::from_rotation_z(vtail_cant);
            parent.spawn((
                Rudder {
                    base_rotation: left_cant,
                },
                Mesh3d(meshes.add(Cuboid::new(0.06, 2.20, 0.55))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(-1.30, 2.10, vtail_z + 1.75).with_rotation(left_cant),
            ));

            // Right rudder
            let right_cant = Quat::from_rotation_z(-vtail_cant);
            parent.spawn((
                Rudder {
                    base_rotation: right_cant,
                },
                Mesh3d(meshes.add(Cuboid::new(0.06, 2.20, 0.55))),
                MeshMaterial3d(ctrl_surface_mat.clone()),
                Transform::from_xyz(1.30, 2.10, vtail_z + 1.75).with_rotation(right_cant),
            ));
        });
}
