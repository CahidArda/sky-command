use bevy::prelude::*;

pub mod hud;
pub mod version;

use crate::camera::CameraSet;

pub struct UiPlugin;

/// System set for UI updates — runs after camera.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSet;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, UiSet.after(CameraSet))
            .add_systems(Startup, (hud::spawn_hud, version::spawn_version_display))
            .add_systems(Update, hud::update_hud.in_set(UiSet));
    }
}
