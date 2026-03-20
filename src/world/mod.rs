use bevy::prelude::*;

pub mod sky;
pub mod terrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (terrain::spawn_terrain, sky::spawn_sky));
    }
}
