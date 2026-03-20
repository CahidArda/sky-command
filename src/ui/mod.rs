use bevy::prelude::*;

pub mod hud;
pub mod menu;
pub mod version;

use crate::camera::CameraSet;
use crate::state::GameState;

pub struct UiPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSet;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, UiSet.after(CameraSet))
            // Menu state
            .add_systems(OnEnter(GameState::Menu), menu::spawn_menu)
            .add_systems(
                Update,
                (menu::handle_menu_buttons, menu::update_button_colors)
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), menu::despawn_menu)
            // Flying state — HUD
            .add_systems(
                OnEnter(GameState::Flying),
                (hud::spawn_hud, version::spawn_version_display),
            )
            .add_systems(
                Update,
                hud::update_hud
                    .in_set(UiSet)
                    .run_if(in_state(GameState::Flying)),
            );
    }
}
