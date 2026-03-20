use bevy::prelude::*;

mod aircraft;
mod camera;
mod input;
mod physics;
mod ui;
mod world;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    run();
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SkyCommand v0.1.0".into(),
                canvas: Some("#skycommand-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            world::WorldPlugin,
            aircraft::AircraftPlugin,
            input::InputPlugin,
            physics::PhysicsPlugin,
            camera::CameraPlugin,
            ui::UiPlugin,
        ))
        .run();
}
