use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Mob {
    pub is_player: bool,
    pub is_champion: bool,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
    pub radius: f32,
    pub last_gate_entity: Option<Entity>,
    pub gate_cooldown: Timer,
}

impl Mob {
    pub fn new_player(speed: f32) -> Self {
        Self {
            is_player: true,
            is_champion: false,
            hp: 1.0,
            max_hp: 1.0,
            speed,
            radius: super::super::core::constants::MOB_RADIUS,
            last_gate_entity: None,
            gate_cooldown: Timer::from_seconds(0.25, TimerMode::Once),
        }
    }

    pub fn new_enemy(speed: f32) -> Self {
        Self {
            is_player: false,
            is_champion: false,
            hp: 1.0,
            max_hp: 1.0,
            speed,
            radius: super::super::core::constants::MOB_RADIUS,
            last_gate_entity: None,
            gate_cooldown: Timer::from_seconds(0.25, TimerMode::Once),
        }
    }

    pub fn new_champion(is_player: bool, hp: f32, speed: f32) -> Self {
        Self {
            is_player,
            is_champion: true,
            hp,
            max_hp: hp,
            speed: speed * 0.75, // Champions move slightly slower and heavier
            radius: super::super::core::constants::CHAMPION_RADIUS,
            last_gate_entity: None,
            gate_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct MobAnimation {
    pub phase: f32,
    pub base_scale: Vec3,
}
