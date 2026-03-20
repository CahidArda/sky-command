use bevy::prelude::*;

mod aircraft;
mod audio;
mod camera;
mod input;
mod physics;
mod state;
mod ui;
mod world;

use state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SkyCommand v0.2.0".into(),
                canvas: Some("#skycommand-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            world::WorldPlugin,
            aircraft::AircraftPlugin,
            input::InputPlugin,
            physics::PhysicsPlugin,
            camera::CameraPlugin,
            ui::UiPlugin,
            audio::AudioPlugin,
        ))
        .run();
}
