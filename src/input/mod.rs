use bevy::prelude::*;

pub mod keyboard;

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
