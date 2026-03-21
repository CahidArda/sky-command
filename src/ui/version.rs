use bevy::prelude::*;

/// Spawn the version display in the bottom-right corner.
pub fn spawn_version_display(mut commands: Commands) {
    commands.spawn((
        Text::new("SkySim.rs v0.2.0 (wasm)"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgba(0.0, 1.0, 0.0, 0.6)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
    ));
}
