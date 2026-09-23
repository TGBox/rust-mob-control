pub mod constants;
pub mod game_state;
pub mod spatial_hash;

use bevy::prelude::*;
pub use constants::*;
pub use game_state::*;
pub use spatial_hash::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(SpatialHash2D::new());
    }
}
