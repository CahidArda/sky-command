use bevy::prelude::*;

use crate::aircraft::{Aircraft, Crashed, SelectedAircraft};
use crate::physics::flight_model::STALL_ANGLE;

/// Marker on all UI entities spawned during Flying state (for cleanup on exit).
#[derive(Component)]
pub struct FlyingUi;

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
pub struct HudVSpeed;
#[derive(Component)]
pub struct HudGLoad;
#[derive(Component)]
pub struct HudStallWarning;
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

    // Aircraft name — centered at top
    commands
        .spawn((
            FlyingUi,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(15.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                HudAircraftName,
                Text::new(selected.0.name()),
                small_font.clone(),
                hud_color,
            ));
        });

    // Speed
    commands.spawn((
        FlyingUi,
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
        FlyingUi,
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
        FlyingUi,
        HudHeading,
        Text::new("HDG: 000deg"),
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
        FlyingUi,
        HudPitch,
        Text::new("PIT: 0.0deg"),
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
        FlyingUi,
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
        FlyingUi,
        HudAoA,
        Text::new("AoA: 0.0deg"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(165.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Vertical speed / sink rate — always shown
    commands.spawn((
        FlyingUi,
        HudVSpeed,
        Text::new("V/S: 0 fpm"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(195.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // G-load
    commands.spawn((
        FlyingUi,
        HudGLoad,
        Text::new("G: 1.0"),
        hud_font,
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(225.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Stall warning — centered in a full-width container
    commands
        .spawn((
            FlyingUi,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                HudStallWarning,
                Text::new(""),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.1)),
            ));
        });

    // Weapons display — only for combat aircraft (below AoA)
    if selected.0.has_weapons() {
        commands.spawn((
            FlyingUi,
            HudWeapons,
            Text::new(format!("WPN: {}", selected.0.weapons_list())),
            small_font,
            TextColor(Color::srgb(1.0, 0.6, 0.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(260.0),
                left: Val::Px(15.0),
                ..default()
            },
        ));
    }
}

/// Update HUD text from aircraft state.
#[allow(clippy::type_complexity)]
pub fn update_hud(
    time: Res<Time>,
    aircraft_query: Query<(&Aircraft, &Transform, Option<&Crashed>)>,
    mut text_query: Query<(
        &mut Text,
        Option<&HudSpeed>,
        Option<&HudAltitude>,
        Option<&HudHeading>,
        Option<&HudPitch>,
        Option<&HudThrottle>,
        Option<&HudAoA>,
        Option<&HudVSpeed>,
        Option<&HudGLoad>,
        Option<&HudStallWarning>,
    )>,
) {
    let Ok((aircraft, transform, crashed)) = aircraft_query.get_single() else {
        return;
    };

    let is_crashed = crashed.is_some();
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
    // Vertical speed: velocity Y component in feet per minute
    let vspeed_fpm = aircraft.velocity.y * M_TO_FEET * 60.0;
    let stalling = aircraft.alpha.abs() > STALL_ANGLE;
    // Blink the stall warning using time
    let show_stall = stalling && ((time.elapsed_secs() * 4.0) as u32).is_multiple_of(2);

    let g_load = aircraft.g_load;

    for (mut text, spd, alt, hdg, pit, thr, aoa, vs, gload, stall) in text_query.iter_mut() {
        if spd.is_some() {
            **text = format!("SPD: {:.0} kts", speed_knots);
        }
        if alt.is_some() {
            **text = format!("ALT: {:.0} ft", altitude_feet);
        }
        if hdg.is_some() {
            **text = format!("HDG: {:03.0}deg", heading_deg);
        }
        if pit.is_some() {
            **text = format!("PIT: {:.1}deg", pitch_deg);
        }
        if thr.is_some() {
            **text = format!("THR: {:.0}%", throttle_pct);
        }
        if aoa.is_some() {
            **text = format!("AoA: {:.1}deg", aoa_deg);
        }
        if vs.is_some() {
            **text = format!("V/S: {:.0} fpm", vspeed_fpm);
        }
        if gload.is_some() {
            **text = format!("G: {:.1}", g_load);
        }
        if stall.is_some() {
            if is_crashed {
                // Blink the crash message
                let show_crash = ((time.elapsed_secs() * 3.0) as u32).is_multiple_of(2);
                **text = if show_crash {
                    "CRASHED - PRESS ESC TO RESTART".to_string()
                } else {
                    String::new()
                };
            } else {
                **text = if show_stall {
                    "STALL".to_string()
                } else {
                    String::new()
                };
            }
        }
    }
}
