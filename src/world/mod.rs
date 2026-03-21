use bevy::prelude::*;

pub mod sky;
pub mod terrain;
pub mod trails;

use crate::physics::PhysicsSet;
use crate::state::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sky::spawn_sky)
            .add_systems(
                OnEnter(GameState::Flying),
                (terrain::init_terrain, trails::init_trail_resources),
            )
            .add_systems(
                Update,
                (
                    terrain::update_terrain.run_if(in_state(GameState::Flying)),
                    (
                        trails::spawn_trail_points,
                        trails::despawn_expired_trail_points,
                    )
                        .after(PhysicsSet::TransformSync)
                        .run_if(in_state(GameState::Flying)),
                ),
            )
            .add_systems(
                OnExit(GameState::Flying),
                (terrain::cleanup_terrain, trails::cleanup_trails),
            );
    }
}
