use bevy::prelude::*;

#[derive(Component)]
pub struct Cannon {
    pub fire_timer: Timer,
    pub champion_charge: f32,
    pub recoil: f32,
}

impl Default for Cannon {
    fn default() -> Self {
        Self {
            fire_timer: Timer::from_seconds(0.14, TimerMode::Repeating),
            champion_charge: 0.0,
            recoil: 0.0,
        }
    }
}

#[derive(Component)]
pub struct CannonBarrel {
    pub offset_x: f32,
}

#[allow(dead_code)]
#[derive(Event)]
pub struct SpawnMobEvent {
    pub position: Vec3,
    pub direction: Vec3,
    pub is_player: bool,
    pub is_champion: bool,
}
