mod audio;
mod cannon;
mod core;
mod gates;
mod juice;
mod level;
mod mobs;
mod ui;
mod upgrades;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mob Control - Rust Edition".into(),
                resolution: (480.0_f32, 854.0_f32).into(), // Classic 9:16 mobile portrait aspect ratio
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(core::CorePlugin)
        .add_plugins(audio::AudioPlugin)
        .add_plugins(upgrades::UpgradesPlugin)
        .add_plugins(cannon::CannonPlugin)
        .add_plugins(mobs::MobsPlugin)
        .add_plugins(gates::GatesPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(juice::JuicePlugin)
        .add_plugins(ui::UiPlugin)
        .run();
}
