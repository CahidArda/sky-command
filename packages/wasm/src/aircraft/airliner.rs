use bevy::prelude::*;
use std::f32::consts::PI;

use super::{Aircraft, ControlInput};

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
        alpha: 0.0,    }
}

/// Spawn the airliner at altitude 1000m heading north.
///
/// The mesh is modeled with the nose at -Z (Bevy's forward convention).
/// Nose faces -Z, tail faces +Z.
///
/// Realistic Boeing 737-800 proportions:
///   Wingspan ~34m, fuselage length ~38m, fuselage diameter ~3.8m
///   LOW WING with swept planform (~35 deg), twin underwing engines,
///   conventional tail, tricycle landing gear.
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
    let off_white_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.90, 0.90, 0.90),
        ..default()
    });
    let wing_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.88, 0.88, 0.90),
        ..default()
    });
    let dark_grey_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.30, 0.33),
        ..default()
    });
    let windshield_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.15, 0.15, 0.18, 0.55),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let accent_red_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.80, 0.10, 0.10),
        ..default()
    });
    let accent_blue_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.22, 0.58),
        ..default()
    });
    let engine_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.55, 0.58),
        ..default()
    });
    let engine_intake_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.20, 0.20, 0.22),
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

    // ── Fuselage reference frame ───────────────────────────────────────
    // Total fuselage: ~38m along Z.  Nose at roughly Z = -19, tail at Z = +19.
    // Root entity placed at approximate CG (~25% MAC), which sits near Z = -2.
    // Fuselage diameter ~3.8m. Y = 0 is fuselage centerline.
    // Wings are LOW — wing top surface at Y ≈ -0.6 (below centerline).

    let root_mesh = meshes.add(Cuboid::new(3.80, 3.80, 10.0));

    // Trim angle of attack at cruise: alpha ~ W/(q*S*Cl_alpha)
    // At 230 m/s: alpha ~ (62000*9.81)/(0.5*1.225*230^2*124.6*2*PI) ~ 0.020 rad
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
            // ── FUSELAGE SECTIONS (tapered, nose at -Z) ────────────────
            // The root cuboid covers the main cabin from Z = -5 to Z = +5.

            // Forward cabin section
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.70, 3.70, 4.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -7.0),
            ));

            // Forward fuselage taper 1
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.50, 3.50, 3.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -10.5),
            ));

            // Nose section — taper 2
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.20, 3.20, 2.5))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -13.25),
            ));

            // Nose cone — taper 3
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.70, 2.70, 2.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -15.5),
            ));

            // Nose tip — radome
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.00, 2.00, 1.5))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -17.25),
            ));

            // Radome cap
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.20, 1.20, 0.8))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -18.40),
            ));

            // Aft fuselage section 1
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.70, 3.70, 4.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.0, 7.0),
            ));

            // Aft fuselage taper 1
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.40, 3.40, 3.0))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.10, 10.5),
            ));

            // Aft fuselage taper 2
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.80, 2.80, 2.5))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 0.20, 13.25),
            ));

            // Tail cone taper 3
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.00, 2.00, 2.0))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(0.0, 0.30, 15.5),
            ));

            // Tail cone tip — APU exhaust area
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.20, 1.20, 1.5))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(0.0, 0.40, 17.25),
            ));

            // ── COCKPIT WINDOWS ──────────────────────────────────────────
            // Wraparound windshield panels on the nose
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 1.40, 0.06))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(0.0, 0.60, -16.53)
                    .with_rotation(Quat::from_rotation_x(-0.30)),
            ));

            // Left cockpit side window
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.05, 0.80, 1.20))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(-1.55, 0.60, -15.5),
            ));

            // Right cockpit side window
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.05, 0.80, 1.20))),
                MeshMaterial3d(windshield_mat.clone()),
                Transform::from_xyz(1.55, 0.60, -15.5),
            ));

            // ── FUSELAGE ACCENT STRIPES ──────────────────────────────────
            // Blue cheatline stripe running along both sides
            let stripe_y = 0.30;
            let stripe_thickness = 0.03;
            let stripe_height_blue = 0.25;
            let stripe_height_red = 0.12;

            // Blue stripe segments — both sides
            for &(z, half_len, half_w) in &[
                (-15.5, 1.5, 1.35),
                (-13.25, 1.25, 1.62),
                (-10.5, 1.50, 1.77),
                (-7.0, 2.0, 1.87),
                (0.0, 5.0, 1.92),
                (7.0, 2.0, 1.87),
                (10.5, 1.50, 1.72),
                (13.25, 1.25, 1.42),
            ] {
                // Left side
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        stripe_height_blue,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent_blue_mat.clone()),
                    Transform::from_xyz(-half_w, stripe_y, z),
                ));
                // Right side
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        stripe_height_blue,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent_blue_mat.clone()),
                    Transform::from_xyz(half_w, stripe_y, z),
                ));
            }

            // Red accent stripe — thinner, just below the blue
            for &(z, half_len, half_w) in &[
                (-13.25, 1.25, 1.62),
                (-10.5, 1.50, 1.77),
                (-7.0, 2.0, 1.87),
                (0.0, 5.0, 1.92),
                (7.0, 2.0, 1.87),
                (10.5, 1.50, 1.72),
            ] {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        stripe_height_red,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent_red_mat.clone()),
                    Transform::from_xyz(-half_w, stripe_y - 0.20, z),
                ));
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        stripe_thickness,
                        stripe_height_red,
                        half_len * 2.0,
                    ))),
                    MeshMaterial3d(accent_red_mat.clone()),
                    Transform::from_xyz(half_w, stripe_y - 0.20, z),
                ));
            }

            // ── LOW WINGS (swept ~35 deg) ────────────────────────────────
            // Wing root is at Y = -0.60 (below fuselage center), Z ~ -1.0.
            // Each wing half spans ~15m outboard. Chord ~4.5m at root, ~2.5m at tip.
            // We approximate the sweep with offset Z positions for inner/outer panels.
            let wing_y = -0.60;
            let wing_root_z = -1.0;

            // Left wing — inner panel (root to ~7m outboard)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.40, 4.20))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-5.40, wing_y, wing_root_z + 1.5),
            ));

            // Left wing — outer panel (7m to ~17m outboard, swept back)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.30, 3.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-12.40, wing_y + 0.30, wing_root_z + 3.5),
            ));

            // Left wing tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.50, 0.20, 2.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(-16.50, wing_y + 0.50, wing_root_z + 4.5),
            ));

            // Right wing — inner panel
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.40, 4.20))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(5.40, wing_y, wing_root_z + 1.5),
            ));

            // Right wing — outer panel
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(7.0, 0.30, 3.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(12.40, wing_y + 0.30, wing_root_z + 3.5),
            ));

            // Right wing tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.50, 0.20, 2.00))),
                MeshMaterial3d(wing_mat.clone()),
                Transform::from_xyz(16.50, wing_y + 0.50, wing_root_z + 4.5),
            ));

            // Wing root fairing — blends wing into fuselage belly
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.00, 0.50, 5.00))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(0.0, wing_y + 0.10, wing_root_z + 1.0),
            ));

            // ── ENGINE NACELLES (underwing, on pylons) ───────────────────
            // 737 engines hang below and forward of the wing on short pylons.
            // Engine center ~5.5m outboard, Y below wing.
            let engine_outboard = 5.80;
            let engine_y = wing_y - 1.60; // well below wing
            let engine_z = wing_root_z - 0.5; // forward of wing leading edge

            // -- Left engine nacelle --
            // Main nacelle body (cylinder approximation with cuboids)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.70, 1.70, 4.5))),
                MeshMaterial3d(engine_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z),
            ));

            // Nacelle front — intake ring (larger diameter)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.90, 1.90, 0.40))),
                MeshMaterial3d(engine_intake_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z - 2.45),
            ));

            // Nacelle intake face (dark)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.60, 1.60, 0.10))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z - 2.70),
            ));

            // Nacelle rear — exhaust cone
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.40, 1.40, 0.80))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y, engine_z + 2.65),
            ));

            // Left pylon — connects nacelle to wing
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 1.20, 2.50))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(-engine_outboard, engine_y + 1.40, engine_z + 0.5),
            ));

            // -- Right engine nacelle --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.70, 1.70, 4.5))),
                MeshMaterial3d(engine_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z),
            ));

            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.90, 1.90, 0.40))),
                MeshMaterial3d(engine_intake_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z - 2.45),
            ));

            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.60, 1.60, 0.10))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z - 2.70),
            ));

            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.40, 1.40, 0.80))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y, engine_z + 2.65),
            ));

            // Right pylon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 1.20, 2.50))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(engine_outboard, engine_y + 1.40, engine_z + 0.5),
            ));

            // ── VERTICAL STABILIZER (conventional tail) ──────────────────
            // Tall swept fin on top of the aft fuselage.
            // Base at Z ~ 14.5, top at Z ~ 17.5.  Height ~ 6m above fuselage top.

            // Main fin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.20, 5.50, 4.50))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 3.80, 15.0),
            ));

            // Upper fin tip — narrower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 1.80, 2.50))),
                MeshMaterial3d(white_mat.clone()),
                Transform::from_xyz(0.0, 7.40, 14.0),
            ));

            // Dorsal fin fillet — blends fin into fuselage
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.16, 1.00, 2.00))),
                MeshMaterial3d(off_white_mat.clone()),
                Transform::from_xyz(0.0, 1.50, 13.5),
            ));

            // Tail logo / accent on vertical stabilizer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.24, 2.50, 2.80))),
                MeshMaterial3d(accent_blue_mat.clone()),
                Transform::from_xyz(0.0, 5.50, 15.0),
            ));

            // ── HORIZONTAL STABILIZER (swept, conventional) ──────────────
            // Mounted at the base of the vertical fin, Z ~ 16.
            // Span ~12.7m, chord ~3m.

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

            // ── TRICYCLE LANDING GEAR ─────────────────────────────────────
            // 737 has nose gear forward and two main gear assemblies under
            // the fuselage belly, roughly beneath the wings.
            let wheel_mesh = meshes.add(Cylinder::new(0.55, 0.30));
            let small_wheel_mesh = meshes.add(Cylinder::new(0.35, 0.20));

            // -- Nose gear --
            // Strut
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 2.50, 0.18))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(0.0, -3.15, -13.0),
            ));

            // Nose gear wheel pair
            parent.spawn((
                Mesh3d(small_wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-0.25, -4.40, -13.0)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
            parent.spawn((
                Mesh3d(small_wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(0.25, -4.40, -13.0)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // -- Left main gear --
            // Strut
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.22, 2.80, 0.22))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-2.20, -3.30, -0.5),
            ));

            // Left main gear truck / axle
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.60, 0.15, 1.60))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-2.20, -4.60, -0.5),
            ));

            // Left main wheels (dual tandem — 2 wheels)
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-2.20, -4.70, -1.10)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-2.20, -4.70, 0.10)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // -- Right main gear --
            // Strut
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.22, 2.80, 0.22))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(2.20, -3.30, -0.5),
            ));

            // Right main gear truck / axle
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.60, 0.15, 1.60))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(2.20, -4.60, -0.5),
            ));

            // Right main wheels (dual tandem — 2 wheels)
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(2.20, -4.70, -1.10)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(2.20, -4.70, 0.10)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));
        });
}
