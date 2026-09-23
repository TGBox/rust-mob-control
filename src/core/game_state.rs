use bevy::prelude::*;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    LevelWon,
    GameOver,
    Shop,
}
