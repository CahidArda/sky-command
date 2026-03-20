use bevy::prelude::*;

/// Top-level game states.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// Aircraft selection menu.
    #[default]
    Menu,
    /// In-flight gameplay.
    Flying,
}
