use bevy::prelude::*;
use std::f32::consts::PI;

use super::{Aircraft, ControlInput};

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
        g_load: 1.0,    }
}

/// Spawn an F-15 Eagle at altitude 1000 m heading north.
///
/// The mesh is modeled with the nose at -Z (Bevy's forward convention).
/// Nose faces -Z, tail faces +Z.
///
/// Realistic F-15 proportions:
///   Length ~19.4 m, wingspan ~13 m, height ~5.6 m
///   Twin-tail, twin-engine, wide flat fuselage, bubble canopy.
pub fn spawn_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Materials ──────────────────────────────────────────────────────
    let grey_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.62, 0.63, 0.64),
        ..default()
    });
    let light_grey_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.70, 0.71, 0.72),
        ..default()
    });
    let dark_grey_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.36, 0.37),
        ..default()
    });
    let very_dark_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.20, 0.20, 0.22),
        ..default()
    });
    let canopy_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.35, 0.50, 0.65, 0.50),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let intake_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.28, 0.30),
        ..default()
    });
    let nozzle_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.22, 0.20),
        ..default()
    });
    let radome_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.45, 0.47),
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
    let pylon_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.50, 0.50, 0.52),
        ..default()
    });
    let missile_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.82),
        ..default()
    });

    // ── Reference frame ──────────────────────────────────────────────
    // F-15 length ~19.4 m.  Nose at Z ≈ -9.7, tail at Z ≈ +9.7.
    // CG roughly at Z = 0.  Wings at Z ≈ 0 (mid-mounted).
    // Y = 0 is fuselage centerline (bottom at Y ≈ -0.70).
    // Fuselage is wide & flat: ~2.4 m wide, ~1.4 m tall at widest.

    let root_mesh = meshes.add(Cuboid::new(2.40, 1.40, 5.00));

    let trim_alpha: f32 = 0.010;
    let start_transform = Transform::from_xyz(0.0, 1000.0, 0.0)
        .with_rotation(Quat::from_rotation_y(PI) * Quat::from_rotation_x(trim_alpha));

    commands
        .spawn((
            default_aircraft(),
            ControlInput::default(),
            Mesh3d(root_mesh),
            MeshMaterial3d(grey_mat.clone()),
            start_transform,
        ))
        .with_children(|parent| {
            // ── FUSELAGE — wide, flat cross-section ─────────────────

            // Forward fuselage section (narrowing toward nose)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.10, 1.30, 2.50))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -3.75),
            ));

            // Forward fuselage — further tapering
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.60, 1.10, 2.00))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(0.0, 0.0, -6.00),
            ));

            // Nose section — narrower
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.10, 0.85, 1.50))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(0.0, -0.05, -7.75),
            ));

            // Radome — the very tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.65, 0.55, 1.00))),
                MeshMaterial3d(radome_mat.clone()),
                Transform::from_xyz(0.0, -0.05, -8.90),
            ));

            // Radome tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 0.30, 0.50))),
                MeshMaterial3d(radome_mat.clone()),
                Transform::from_xyz(0.0, -0.05, -9.45),
            ));

            // Aft fuselage — between wings and engines
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.40, 1.35, 3.00))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(0.0, 0.0, 4.00),
            ));

            // Tail section — narrows between the twin engine nacelles
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.00, 1.20, 2.50))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(0.0, 0.0, 6.75),
            ));

            // Engine fairing — aft-most fuselage spine between nacelles
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 0.70, 2.00))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.0, 0.0, 8.50),
            ));

            // ── BUBBLE CANOPY ───────────────────────────────────────
            // F-15 has a single-seat bubble canopy, fairly far forward.

            // Canopy frame / base
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.90, 0.20, 2.20))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.0, 0.75, -5.80),
            ));

            // Canopy glass — the bubble
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 0.45, 2.00))),
                MeshMaterial3d(canopy_mat.clone()),
                Transform::from_xyz(0.0, 1.00, -5.80),
            ));

            // Canopy front (windscreen, angled)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.75, 0.40, 0.04))),
                MeshMaterial3d(canopy_mat.clone()),
                Transform::from_xyz(0.0, 0.90, -6.85)
                    .with_rotation(Quat::from_rotation_x(-0.50)),
            ));

            // Canopy rear fairing — tapers into spine
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.60, 0.30, 1.00))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(0.0, 0.80, -4.50),
            ));

            // ── INTAKE RAMPS — boxy side intakes ────────────────────
            // The F-15 has large rectangular intakes on each side of the fuselage,
            // starting just behind the cockpit area.

            // Left intake
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 1.00, 3.50))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(-1.60, -0.10, -2.75),
            ));

            // Left intake opening face (dark)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.70, 0.85, 0.10))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(-1.60, -0.15, -4.55),
            ));

            // Left intake interior shadow
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.60, 0.75, 0.50))),
                MeshMaterial3d(very_dark_mat.clone()),
                Transform::from_xyz(-1.60, -0.15, -4.25),
            ));

            // Right intake
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.80, 1.00, 3.50))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(1.60, -0.10, -2.75),
            ));

            // Right intake opening face (dark)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.70, 0.85, 0.10))),
                MeshMaterial3d(intake_mat.clone()),
                Transform::from_xyz(1.60, -0.15, -4.55),
            ));

            // Right intake interior shadow
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.60, 0.75, 0.50))),
                MeshMaterial3d(very_dark_mat.clone()),
                Transform::from_xyz(1.60, -0.15, -4.25),
            ));

            // ── WINGS — mid-mounted, trapezoidal / cropped delta ────
            // Wingspan ~13 m total. Root chord ~5 m, tip chord ~1.5 m.
            // Leading edge sweep ~45°. Wings are thin.
            // Each wing panel spans ~5 m from fuselage side (fuselage ~1.2 m half-width).

            let wing_y = -0.15;
            let wing_z = 0.50; // wings are roughly centered, slightly aft

            // Left wing — inner section (wider chord)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 0.12, 3.80))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(-2.50, wing_y, wing_z),
            ));

            // Left wing — outer section (tapered, narrower chord)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 0.10, 2.20))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(-4.70, wing_y, wing_z + 0.80),
            ));

            // Left wing tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.60, 0.08, 1.40))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(-5.90, wing_y, wing_z + 1.30),
            ));

            // Right wing — inner section
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 0.12, 3.80))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(2.50, wing_y, wing_z),
            ));

            // Right wing — outer section
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 0.10, 2.20))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(4.70, wing_y, wing_z + 0.80),
            ));

            // Right wing tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.60, 0.08, 1.40))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(5.90, wing_y, wing_z + 1.30),
            ));

            // ── WING LEADING-EDGE SWEEP ACCENTS ─────────────────────
            // Darker leading edge strips to suggest the sweep angle

            // Left wing leading edge
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 0.14, 0.15))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-2.50, wing_y, wing_z - 1.90),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 0.12, 0.12))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-4.70, wing_y, wing_z + 0.80 - 1.10),
            ));

            // Right wing leading edge
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.60, 0.14, 0.15))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(2.50, wing_y, wing_z - 1.90),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.80, 0.12, 0.12))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(4.70, wing_y, wing_z + 0.80 - 1.10),
            ));

            // ── TWIN VERTICAL STABILIZERS (angled outward ~15°) ─────
            // The F-15's defining feature: two canted vertical tails.
            // Each is ~3.5 m tall, root chord ~3 m, tip chord ~1.2 m.
            // Mounted at the rear of the engine nacelles.

            let vtail_z = 7.00;
            let vtail_cant = 15.0_f32.to_radians(); // outward cant

            // Left vertical stabilizer — main slab
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 2.80, 2.80))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(-1.30, 2.00, vtail_z)
                    .with_rotation(Quat::from_rotation_z(vtail_cant)),
            ));

            // Left vertical stabilizer — tip (narrower)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 0.80, 1.50))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(-1.55, 3.60, vtail_z + 0.40)
                    .with_rotation(Quat::from_rotation_z(vtail_cant)),
            ));

            // Left vertical stab — darker leading edge
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.12, 2.80, 0.12))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-1.30, 2.00, vtail_z - 1.40)
                    .with_rotation(Quat::from_rotation_z(vtail_cant)),
            ));

            // Right vertical stabilizer — main slab
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.10, 2.80, 2.80))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(1.30, 2.00, vtail_z)
                    .with_rotation(Quat::from_rotation_z(-vtail_cant)),
            ));

            // Right vertical stabilizer — tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 0.80, 1.50))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(1.55, 3.60, vtail_z + 0.40)
                    .with_rotation(Quat::from_rotation_z(-vtail_cant)),
            ));

            // Right vertical stab — leading edge
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.12, 2.80, 0.12))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(1.30, 2.00, vtail_z - 1.40)
                    .with_rotation(Quat::from_rotation_z(-vtail_cant)),
            ));

            // ── HORIZONTAL STABILIZERS (all-moving "stabilators") ───
            // The F-15 has all-moving horizontal tails, ~6.2 m span total.
            // Mounted low, at the base of the vertical tails.

            let htail_z = 8.00;
            let htail_y = 0.10;

            // Left horizontal stabilizer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.80, 0.08, 2.00))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(-1.90, htail_y, htail_z),
            ));

            // Left htail tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.70, 0.06, 1.20))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(-3.60, htail_y, htail_z + 0.30),
            ));

            // Right horizontal stabilizer
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.80, 0.08, 2.00))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(1.90, htail_y, htail_z),
            ));

            // Right htail tip
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.70, 0.06, 1.20))),
                MeshMaterial3d(light_grey_mat.clone()),
                Transform::from_xyz(3.60, htail_y, htail_z + 0.30),
            ));

            // ── TWIN ENGINE NACELLES ────────────────────────────────
            // The F-15's engines sit side by side in the aft fuselage.
            // Each nacelle: ~1.0 m wide, extends to the exhaust nozzles.

            let nacelle_y = -0.05;

            // Left engine nacelle
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.05, 1.10, 4.00))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(-0.70, nacelle_y, 5.50),
            ));

            // Right engine nacelle
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.05, 1.10, 4.00))),
                MeshMaterial3d(grey_mat.clone()),
                Transform::from_xyz(0.70, nacelle_y, 5.50),
            ));

            // ── ENGINE NOZZLES ──────────────────────────────────────
            // Circular-ish exhaust nozzles at the very rear.

            let nozzle_z = 9.20;

            // Left nozzle ring
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.45, 0.60))),
                MeshMaterial3d(nozzle_mat.clone()),
                Transform::from_xyz(-0.70, nacelle_y, nozzle_z)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // Left nozzle interior (dark)
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.35, 0.20))),
                MeshMaterial3d(very_dark_mat.clone()),
                Transform::from_xyz(-0.70, nacelle_y, nozzle_z + 0.20)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // Right nozzle ring
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.45, 0.60))),
                MeshMaterial3d(nozzle_mat.clone()),
                Transform::from_xyz(0.70, nacelle_y, nozzle_z)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // Right nozzle interior (dark)
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.35, 0.20))),
                MeshMaterial3d(very_dark_mat.clone()),
                Transform::from_xyz(0.70, nacelle_y, nozzle_z + 0.20)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // ── SPEED BRAKE AREA (dorsal, between the tails) ────────
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.40, 0.06, 2.00))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.0, 0.75, 6.50),
            ));

            // ── WINGTIP MISSILE RAILS ───────────────────────────────
            // Small pylons at the wingtips carrying AIM-9 Sidewinders.

            let rail_y = wing_y - 0.15;

            // Left wingtip pylon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 0.20, 0.60))),
                MeshMaterial3d(pylon_mat.clone()),
                Transform::from_xyz(-6.10, rail_y, wing_z + 1.30),
            ));

            // Left wingtip rail
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.04, 1.20))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-6.10, rail_y - 0.20, wing_z + 1.30),
            ));

            // Left AIM-9 Sidewinder missile body
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.065, 2.80))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(-6.10, rail_y - 0.35, wing_z + 1.30)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // Left missile fins (front canards)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 0.02, 0.10))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(-6.10, rail_y - 0.35, wing_z + 1.30 - 1.10),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.02, 0.30, 0.10))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(-6.10, rail_y - 0.35, wing_z + 1.30 - 1.10),
            ));

            // Left missile tail fins
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.25, 0.02, 0.12))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(-6.10, rail_y - 0.35, wing_z + 1.30 + 1.20),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.02, 0.25, 0.12))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(-6.10, rail_y - 0.35, wing_z + 1.30 + 1.20),
            ));

            // Right wingtip pylon
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 0.20, 0.60))),
                MeshMaterial3d(pylon_mat.clone()),
                Transform::from_xyz(6.10, rail_y, wing_z + 1.30),
            ));

            // Right wingtip rail
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.04, 1.20))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(6.10, rail_y - 0.20, wing_z + 1.30),
            ));

            // Right AIM-9 Sidewinder missile body
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.065, 2.80))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(6.10, rail_y - 0.35, wing_z + 1.30)
                    .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ));

            // Right missile fins (front canards)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.30, 0.02, 0.10))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(6.10, rail_y - 0.35, wing_z + 1.30 - 1.10),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.02, 0.30, 0.10))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(6.10, rail_y - 0.35, wing_z + 1.30 - 1.10),
            ));

            // Right missile tail fins
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.25, 0.02, 0.12))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(6.10, rail_y - 0.35, wing_z + 1.30 + 1.20),
            ));
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.02, 0.25, 0.12))),
                MeshMaterial3d(missile_mat.clone()),
                Transform::from_xyz(6.10, rail_y - 0.35, wing_z + 1.30 + 1.20),
            ));

            // ── TRICYCLE LANDING GEAR ───────────────────────────────
            let wheel_mesh = meshes.add(Cylinder::new(0.22, 0.14));

            // -- Nosewheel strut --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 1.00, 0.08))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(0.0, -1.20, -7.00),
            ));

            // Nosewheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(0.0, -1.72, -7.00)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // -- Left main gear strut --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 1.10, 0.10))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(-1.30, -1.25, 0.50),
            ));

            // Left main wheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(-1.30, -1.82, 0.50)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // -- Right main gear strut --
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 1.10, 0.10))),
                MeshMaterial3d(gear_mat.clone()),
                Transform::from_xyz(1.30, -1.25, 0.50),
            ));

            // Right main wheel
            parent.spawn((
                Mesh3d(wheel_mesh.clone()),
                MeshMaterial3d(tire_mat.clone()),
                Transform::from_xyz(1.30, -1.82, 0.50)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ));

            // ── FUSELAGE PANEL LINES / DETAILS ──────────────────────
            // Subtle darker panels on the upper fuselage for visual interest

            // Dorsal spine — raised ridge running along the top
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.40, 0.10, 8.00))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.0, 0.75, -1.50),
            ));

            // Anti-collision light housing (small bump on the spine)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.12, 0.08, 0.12))),
                MeshMaterial3d(very_dark_mat.clone()),
                Transform::from_xyz(0.0, 0.82, -3.00),
            ));

            // Ventral fins — small stabilizing fins under the aft fuselage
            // Left ventral fin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.50, 0.80))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(-0.70, -1.00, 7.50),
            ));

            // Right ventral fin
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.06, 0.50, 0.80))),
                MeshMaterial3d(dark_grey_mat.clone()),
                Transform::from_xyz(0.70, -1.00, 7.50),
            ));
        });
}
