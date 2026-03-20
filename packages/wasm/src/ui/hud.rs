use bevy::prelude::*;

use crate::aircraft::{Aircraft, SelectedAircraft};

#[derive(Component)]
pub struct HudSpeed;
#[derive(Component)]
pub struct HudAltitude;
#[derive(Component)]
pub struct HudHeading;
#[derive(Component)]
pub struct HudPitch;
#[derive(Component)]
pub struct HudThrottle;
#[derive(Component)]
pub struct HudAoA;
#[derive(Component)]
pub struct HudWeapons;
#[derive(Component)]
pub struct HudAircraftName;

const MS_TO_KNOTS: f32 = 1.94384;
const M_TO_FEET: f32 = 3.28084;

/// Spawn all HUD text entities.
pub fn spawn_hud(mut commands: Commands, selected: Res<SelectedAircraft>) {
    let hud_font = TextFont {
        font_size: 22.0,
        ..default()
    };
    let hud_color = TextColor(Color::srgb(0.0, 1.0, 0.0));
    let small_font = TextFont {
        font_size: 16.0,
        ..default()
    };

    // Aircraft name — top center
    commands.spawn((
        HudAircraftName,
        Text::new(selected.0.name()),
        small_font.clone(),
        TextColor(Color::srgb(0.0, 0.8, 0.3)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Percent(50.0),
            ..default()
        },
    ));

    // Speed
    commands.spawn((
        HudSpeed,
        Text::new("SPD: 0 kts"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Altitude
    commands.spawn((
        HudAltitude,
        Text::new("ALT: 0 ft"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(45.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Heading
    commands.spawn((
        HudHeading,
        Text::new("HDG: 000\u{00B0}"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(75.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Pitch
    commands.spawn((
        HudPitch,
        Text::new("PIT: 0.0\u{00B0}"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(105.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Throttle
    commands.spawn((
        HudThrottle,
        Text::new("THR: 0%"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(135.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Angle of Attack
    commands.spawn((
        HudAoA,
        Text::new("AoA: 0.0\u{00B0}"),
        hud_font,
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(165.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Weapons display — only for combat aircraft (below AoA)
    if selected.0.has_weapons() {
        commands.spawn((
            HudWeapons,
            Text::new(format!("WPN: {}", selected.0.weapons_list())),
            small_font,
            TextColor(Color::srgb(1.0, 0.6, 0.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(200.0),
                left: Val::Px(15.0),
                ..default()
            },
        ));
    }
}

/// Update HUD text from aircraft state.
pub fn update_hud(
    aircraft_query: Query<(&Aircraft, &Transform)>,
    mut speed_q: Query<&mut Text, (With<HudSpeed>, Without<HudAltitude>, Without<HudHeading>, Without<HudPitch>, Without<HudThrottle>, Without<HudAoA>)>,
    mut alt_q: Query<&mut Text, (With<HudAltitude>, Without<HudSpeed>, Without<HudHeading>, Without<HudPitch>, Without<HudThrottle>, Without<HudAoA>)>,
    mut hdg_q: Query<&mut Text, (With<HudHeading>, Without<HudSpeed>, Without<HudAltitude>, Without<HudPitch>, Without<HudThrottle>, Without<HudAoA>)>,
    mut pit_q: Query<&mut Text, (With<HudPitch>, Without<HudSpeed>, Without<HudAltitude>, Without<HudHeading>, Without<HudThrottle>, Without<HudAoA>)>,
    mut thr_q: Query<&mut Text, (With<HudThrottle>, Without<HudSpeed>, Without<HudAltitude>, Without<HudHeading>, Without<HudPitch>, Without<HudAoA>)>,
    mut aoa_q: Query<&mut Text, (With<HudAoA>, Without<HudSpeed>, Without<HudAltitude>, Without<HudHeading>, Without<HudPitch>, Without<HudThrottle>)>,
) {
    let Ok((aircraft, transform)) = aircraft_query.get_single() else {
        return;
    };

    let speed_knots = aircraft.velocity.length() * MS_TO_KNOTS;
    let altitude_feet = transform.translation.y * M_TO_FEET;
    let forward = transform.forward().as_vec3();
    let heading_rad = forward.x.atan2(forward.z);
    let mut heading_deg = heading_rad.to_degrees();
    if heading_deg < 0.0 {
        heading_deg += 360.0;
    }
    let pitch_deg = forward.y.asin().to_degrees();
    let throttle_pct = aircraft.throttle * 100.0;
    let aoa_deg = aircraft.alpha.to_degrees();

    for mut t in speed_q.iter_mut() { **t = format!("SPD: {:.0} kts", speed_knots); }
    for mut t in alt_q.iter_mut() { **t = format!("ALT: {:.0} ft", altitude_feet); }
    for mut t in hdg_q.iter_mut() { **t = format!("HDG: {:03.0}\u{00B0}", heading_deg); }
    for mut t in pit_q.iter_mut() { **t = format!("PIT: {:.1}\u{00B0}", pitch_deg); }
    for mut t in thr_q.iter_mut() { **t = format!("THR: {:.0}%", throttle_pct); }
    for mut t in aoa_q.iter_mut() { **t = format!("AoA: {:.1}\u{00B0}", aoa_deg); }
}
