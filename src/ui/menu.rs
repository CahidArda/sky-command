use bevy::prelude::*;

use crate::aircraft::{AircraftType, SelectedAircraft};
use crate::state::GameState;

/// Marker for the entire menu UI root so we can despawn it.
#[derive(Component)]
pub struct MenuRoot;

/// Marker for a selectable aircraft button, storing which type it represents.
#[derive(Component)]
pub struct AircraftButton(pub AircraftType);

/// Spawn the aircraft selection menu.
pub fn spawn_menu(mut commands: Commands) {
    let aircraft_types = [
        AircraftType::Prop,
        AircraftType::Airliner,
        AircraftType::Fighter,
    ];

    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("SkySim.rs"),
                TextFont {
                    font_size: 52.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));

            // Subtitle
            root.spawn((
                Text::new("Select Your Aircraft"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Aircraft buttons
            for aircraft_type in aircraft_types {
                root.spawn((
                    AircraftButton(aircraft_type),
                    Button,
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(70.0),
                        margin: UiRect::all(Val::Px(6.0)),
                        padding: UiRect::horizontal(Val::Px(20.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(6.0)),
                    BackgroundColor(Color::srgb(0.12, 0.14, 0.18)),
                ))
                .with_children(|btn| {
                    // Aircraft name
                    btn.spawn((
                        Text::new(aircraft_type.name()),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Description
                    btn.spawn((
                        Text::new(aircraft_type.description()),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.6, 0.7)),
                    ));
                });
            }

            // Controls hint
            root.spawn((
                Text::new(
                    "W/S Pitch · A/D Roll · Q/E Yaw · Shift/Ctrl Throttle · C Camera · ESC Menu",
                ),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.3, 0.4)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Version
            root.spawn((
                Text::new("v0.2.0"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.3, 0.4)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

/// Handle button clicks — select aircraft and transition to Flying.
#[allow(clippy::type_complexity)]
pub fn handle_menu_buttons(
    interaction_query: Query<(&Interaction, &AircraftButton), (Changed<Interaction>, With<Button>)>,
    mut selected: ResMut<SelectedAircraft>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, aircraft_btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            selected.0 = aircraft_btn.0;
            next_state.set(GameState::Flying);
        }
    }
}

/// Update button colors on hover.
#[allow(clippy::type_complexity)]
pub fn update_button_colors(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AircraftButton>),
    >,
) {
    for (interaction, mut bg) in query.iter_mut() {
        *bg = match interaction {
            Interaction::Pressed => BackgroundColor(Color::srgb(0.2, 0.4, 0.7)),
            Interaction::Hovered => BackgroundColor(Color::srgb(0.18, 0.22, 0.30)),
            Interaction::None => BackgroundColor(Color::srgb(0.12, 0.14, 0.18)),
        };
    }
}

/// Despawn the menu when leaving Menu state.
pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
