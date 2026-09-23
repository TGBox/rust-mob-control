pub mod resources;

use bevy::prelude::*;
pub use resources::*;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStats>()
            .init_resource::<Upgrades>();
    }
}
