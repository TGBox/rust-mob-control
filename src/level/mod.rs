pub mod components;
pub mod levels;
pub mod systems;

use bevy::prelude::*;
use crate::core::GameState;
use systems::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_global_camera_and_lighting)
            .add_systems(OnEnter(GameState::Playing), setup_level)
            .add_systems(
                Update,
                (
                    update_enemy_spawner,
                    update_base_combat_and_win,
                    update_defeat_condition,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_level);
    }
}
