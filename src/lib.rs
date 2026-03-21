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

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    run();
}

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SkySim.rs v0.2.0".into(),
                        canvas: Some("#skysim-canvas".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
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
