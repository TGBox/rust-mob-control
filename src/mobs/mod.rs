pub mod components;
pub mod systems;

use bevy::prelude::*;
pub use components::*;
pub use systems::*;
use crate::core::GameState;

pub struct MobsPlugin;

impl Plugin for MobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_mob_assets)
            .add_systems(
                Update,
                (
                    handle_spawn_mob_events,
                    update_spatial_hash,
                    update_mob_movement_and_separation,
                    update_mob_combat,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_mobs);
    }
}
