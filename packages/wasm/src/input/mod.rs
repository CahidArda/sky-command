use bevy::prelude::*;

pub mod keyboard;

pub struct InputPlugin;

/// System set for input handling — runs before physics.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            InputSet.before(crate::physics::PhysicsSet::Forces),
        )
        .add_systems(Update, keyboard::handle_keyboard_input.in_set(InputSet));
    }
}
