pub mod components;
pub mod systems;

use bevy::prelude::*;
pub use components::*;
pub use systems::*;
use crate::core::GameState;

pub struct GatesPlugin;

impl Plugin for GatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_gate_movement, update_gate_mob_collisions)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_gates);
    }
}
