pub mod components;
pub mod systems;

use bevy::prelude::*;
pub use components::*;
pub use systems::*;
use crate::core::GameState;

pub struct CannonPlugin;

impl Plugin for CannonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnMobEvent>()
            .add_systems(OnEnter(GameState::Playing), setup_cannon)
            .add_systems(
                Update,
                (update_cannon_movement, update_cannon_shooting)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_cannon);
    }
}
