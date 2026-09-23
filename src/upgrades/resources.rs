use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct GameStats {
    pub coins: u32,
    pub current_level: usize,
    pub mobs_spawned_total: u32,
    pub enemies_defeated_total: u32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            coins: 50, // Starting bonus
            current_level: 1,
            mobs_spawned_total: 0,
            enemies_defeated_total: 0,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Upgrades {
    pub fire_rate_level: u32,
    pub mob_speed_level: u32,
    pub cannon_barrels: u32, // 1, 2, or 3
    pub champion_level: u32,
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            fire_rate_level: 1,
            mob_speed_level: 1,
            cannon_barrels: 1,
            champion_level: 1,
        }
    }
}

impl Upgrades {
    pub fn fire_rate_cost(&self) -> u32 {
        25 * self.fire_rate_level
    }

    pub fn mob_speed_cost(&self) -> u32 {
        30 * self.mob_speed_level
    }

    pub fn barrel_cost(&self) -> u32 {
        match self.cannon_barrels {
            1 => 120,
            2 => 300,
            _ => 9999,
        }
    }

    pub fn champion_cost(&self) -> u32 {
        50 * self.champion_level
    }

    pub fn shots_per_second(&self) -> f32 {
        super::super::core::constants::CANNON_BASE_FIRE_RATE
            + (self.fire_rate_level - 1) as f32 * 1.5
    }

    pub fn mob_speed(&self) -> f32 {
        super::super::core::constants::MOB_BASE_SPEED
            + (self.mob_speed_level - 1) as f32 * 1.2
    }

    pub fn champion_hp(&self) -> f32 {
        super::super::core::constants::CHAMPION_BASE_HP
            + (self.champion_level - 1) as f32 * 10.0
    }
}
