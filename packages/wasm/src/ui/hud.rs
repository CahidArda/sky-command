use bevy::prelude::*;

use crate::aircraft::Aircraft;

/// Marker for the speed text element.
#[derive(Component)]
pub struct HudSpeed;

/// Marker for the altitude text element.
#[derive(Component)]
pub struct HudAltitude;

/// Marker for the heading text element.
#[derive(Component)]
pub struct HudHeading;

/// Marker for the pitch text element.
#[derive(Component)]
pub struct HudPitch;

/// Marker for the throttle text element.
#[derive(Component)]
pub struct HudThrottle;

/// Conversion factor: m/s to knots.
const MS_TO_KNOTS: f32 = 1.94384;

/// Conversion factor: meters to feet.
const M_TO_FEET: f32 = 3.28084;

/// Spawn all HUD text entities.
pub fn spawn_hud(mut commands: Commands) {
    let hud_font = TextFont {
        font_size: 22.0,
        ..default()
    };
    let hud_color = TextColor(Color::srgb(0.0, 1.0, 0.0));

    // Speed indicator — top left
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

    // Altitude indicator — below speed
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

    // Heading indicator — below altitude
    commands.spawn((
        HudHeading,
        Text::new("HDG: 000°"),
        hud_font.clone(),
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(75.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    // Pitch indicator — below heading
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

    // Throttle indicator — below pitch
    commands.spawn((
        HudThrottle,
        Text::new("THR: 0%"),
        hud_font,
        hud_color,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(135.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));
}

/// Update all HUD text elements from the aircraft state.
pub fn update_hud(
    aircraft_query: Query<(&Aircraft, &Transform)>,
    mut speed_query: Query<&mut Text, (With<HudSpeed>, Without<HudAltitude>, Without<HudHeading>, Without<HudPitch>, Without<HudThrottle>)>,
    mut alt_query: Query<&mut Text, (With<HudAltitude>, Without<HudSpeed>, Without<HudHeading>, Without<HudPitch>, Without<HudThrottle>)>,
    mut hdg_query: Query<&mut Text, (With<HudHeading>, Without<HudSpeed>, Without<HudAltitude>, Without<HudPitch>, Without<HudThrottle>)>,
    mut pit_query: Query<&mut Text, (With<HudPitch>, Without<HudSpeed>, Without<HudAltitude>, Without<HudHeading>, Without<HudThrottle>)>,
    mut thr_query: Query<&mut Text, (With<HudThrottle>, Without<HudSpeed>, Without<HudAltitude>, Without<HudHeading>, Without<HudPitch>)>,
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

    // Pitch: angle of forward above/below the horizon
    let pitch_deg = forward.y.asin().to_degrees();

    let throttle_pct = aircraft.throttle * 100.0;

    for mut text in speed_query.iter_mut() {
        **text = format!("SPD: {:.0} kts", speed_knots);
    }
    for mut text in alt_query.iter_mut() {
        **text = format!("ALT: {:.0} ft", altitude_feet);
    }
    for mut text in hdg_query.iter_mut() {
        **text = format!("HDG: {:03.0}\u{00B0}", heading_deg);
    }
    for mut text in pit_query.iter_mut() {
        **text = format!("PIT: {:.1}\u{00B0}", pitch_deg);
    }
    for mut text in thr_query.iter_mut() {
        **text = format!("THR: {:.0}%", throttle_pct);
    }
}
