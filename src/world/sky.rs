use bevy::prelude::*;

/// Spawn the sky environment: a directional light (sun) and ambient light.
/// Bevy's default clear color serves as the sky background.
pub fn spawn_sky(mut commands: Commands) {
    // Set a sky-blue clear color as the background
    commands.insert_resource(ClearColor(Color::srgb(0.53, 0.77, 0.97)));

    // Ambient light for general illumination
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.9, 0.92, 1.0),
        brightness: 500.0,
    });

    // Directional light — the sun
    // Positioned to cast light from upper-right, slightly behind
    commands.spawn((
        DirectionalLight {
            illuminance: 12000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -0.8, // tilt down
            0.4,  // rotate around Y
            0.0,
        )),
    ));
}
