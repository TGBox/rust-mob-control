pub mod hud;
pub mod menus;

use bevy::prelude::*;
pub use hud::*;
pub use menus::*;
use crate::core::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Main Menu
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            // In-Game HUD
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(Update, update_hud.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_hud)
            // Victory Menu
            .add_systems(OnEnter(GameState::LevelWon), setup_victory_menu)
            .add_systems(OnExit(GameState::LevelWon), cleanup_menu)
            // Defeat Menu
            .add_systems(OnEnter(GameState::GameOver), setup_game_over_menu)
            .add_systems(OnExit(GameState::GameOver), cleanup_menu)
            // Shop Menu
            .add_systems(OnEnter(GameState::Shop), setup_shop_menu)
            .add_systems(OnExit(GameState::Shop), cleanup_menu)
            // Shared button interactions
            .add_systems(Update, handle_menu_buttons);
    }
}
