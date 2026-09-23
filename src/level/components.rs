use bevy::prelude::*;

#[derive(Component)]
pub struct LevelEnvironment;

#[derive(Component)]
pub struct EnemyBase {
    pub current_hp: f32,
    pub max_hp: f32,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct EnemySpawner {
    pub spawn_timer: Timer,
    pub champion_timer: Timer,
    pub mobs_per_wave: u32,
    pub waves_remaining: u32,
}

#[derive(Component)]
pub struct BaseHpText;
