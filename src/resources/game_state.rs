use bevy::prelude::*;

/// Game states for managing different screens and game flow
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    NewWorldkeeper,
    InGame,
    Paused,
    GameOver,
}

impl GameState {
    /// Check if the current state allows UI interaction
    pub fn allows_ui_interaction(&self) -> bool {
        matches!(self, GameState::MainMenu | GameState::NewWorldkeeper | GameState::Paused)
    }
    
    /// Check if the current state is in active gameplay
    pub fn is_gameplay(&self) -> bool {
        matches!(self, GameState::InGame)
    }
}