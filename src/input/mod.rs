use bevy::prelude::*;

pub mod keyboard;

use crate::aircraft::{Aircraft, Crashed};
use crate::state::GameState;

pub struct InputPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, InputSet.before(crate::physics::PhysicsSet::Forces))
            .add_systems(
                Update,
                (
                    keyboard::handle_keyboard_input.in_set(InputSet),
                    handle_escape,
                    handle_crash_restart,
                )
                    .run_if(in_state(GameState::Flying)),
            );
    }
}

/// Press ESC to return to aircraft selection menu.
fn handle_escape(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

/// Press ESC after a crash to return to the menu.
fn handle_crash_restart(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<&Aircraft, With<Crashed>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) && query.get_single().is_ok() {
        next_state.set(GameState::Menu);
    }
}
