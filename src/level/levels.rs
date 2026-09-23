use bevy::prelude::*;
use crate::gates::{GateMovement, GateType};

pub struct GateConfig {
    pub gate_type: GateType,
    pub movement: GateMovement,
    pub width: f32,
    pub position: Vec3,
}

#[allow(dead_code)]
pub struct LevelDefinition {
    pub base_hp: f32,
    pub spawn_interval: f32,
    pub mobs_per_wave: u32,
    pub has_enemy_champions: bool,
    pub gates: Vec<GateConfig>,
}

pub fn get_level_definition(level: usize) -> LevelDefinition {
    match level {
        1 => LevelDefinition {
            base_hp: 40.0,
            spawn_interval: 2.2,
            mobs_per_wave: 2,
            has_enemy_champions: false,
            gates: vec![GateConfig {
                gate_type: GateType::Multiply(2),
                movement: GateMovement::Static,
                width: 5.5,
                position: Vec3::new(0.0, 0.0, 5.0),
            }],
        },
        2 => LevelDefinition {
            base_hp: 80.0,
            spawn_interval: 1.8,
            mobs_per_wave: 3,
            has_enemy_champions: false,
            gates: vec![
                GateConfig {
                    gate_type: GateType::Multiply(3),
                    movement: GateMovement::Sine {
                        amplitude: 3.5,
                        speed: 2.0,
                        center_x: 0.0,
                        phase: 0.0,
                    },
                    width: 4.5,
                    position: Vec3::new(0.0, 0.0, 7.0),
                },
                GateConfig {
                    gate_type: GateType::Add(5),
                    movement: GateMovement::Static,
                    width: 4.0,
                    position: Vec3::new(0.0, 0.0, -5.0),
                },
            ],
        },
        3 => LevelDefinition {
            base_hp: 140.0,
            spawn_interval: 1.4,
            mobs_per_wave: 4,
            has_enemy_champions: false,
            gates: vec![
                GateConfig {
                    gate_type: GateType::Multiply(2),
                    movement: GateMovement::Sine {
                        amplitude: 2.5,
                        speed: 2.5,
                        center_x: -2.5,
                        phase: 0.0,
                    },
                    width: 4.0,
                    position: Vec3::new(-2.5, 0.0, 8.0),
                },
                GateConfig {
                    gate_type: GateType::Add(8),
                    movement: GateMovement::Sine {
                        amplitude: 2.5,
                        speed: 2.5,
                        center_x: 2.5,
                        phase: std::f32::consts::PI,
                    },
                    width: 4.0,
                    position: Vec3::new(2.5, 0.0, 8.0),
                },
                GateConfig {
                    gate_type: GateType::Multiply(3),
                    movement: GateMovement::Static,
                    width: 5.0,
                    position: Vec3::new(0.0, 0.0, -4.0),
                },
            ],
        },
        4 => LevelDefinition {
            base_hp: 220.0,
            spawn_interval: 1.2,
            mobs_per_wave: 5,
            has_enemy_champions: true,
            gates: vec![
                GateConfig {
                    gate_type: GateType::Multiply(4),
                    movement: GateMovement::Sine {
                        amplitude: 4.0,
                        speed: 3.0,
                        center_x: 0.0,
                        phase: 0.0,
                    },
                    width: 4.0,
                    position: Vec3::new(0.0, 0.0, 9.0),
                },
                GateConfig {
                    gate_type: GateType::Multiply(2),
                    movement: GateMovement::Static,
                    width: 4.5,
                    position: Vec3::new(-3.0, 0.0, -2.0),
                },
                GateConfig {
                    gate_type: GateType::Add(15),
                    movement: GateMovement::Static,
                    width: 4.5,
                    position: Vec3::new(3.0, 0.0, -2.0),
                },
            ],
        },
        _ => {
            // Level 5+ Procedural escalation
            let scaling = (level as f32 - 4.0) * 0.4;
            LevelDefinition {
                base_hp: 300.0 + (level as f32 * 50.0),
                spawn_interval: (1.1 - scaling * 0.1).max(0.6),
                mobs_per_wave: 5 + (level as u32 / 2),
                has_enemy_champions: true,
                gates: vec![
                    GateConfig {
                        gate_type: GateType::Multiply((2 + (level % 3)) as u32),
                        movement: GateMovement::Sine {
                            amplitude: 4.0,
                            speed: 2.5 + (level as f32 * 0.2),
                            center_x: 0.0,
                            phase: 0.0,
                        },
                        width: 4.2,
                        position: Vec3::new(0.0, 0.0, 10.0),
                    },
                    GateConfig {
                        gate_type: GateType::Add(10 + (level as u32 * 2)),
                        movement: GateMovement::Sine {
                            amplitude: 3.5,
                            speed: 2.2,
                            center_x: 0.0,
                            phase: std::f32::consts::FRAC_PI_2,
                        },
                        width: 4.5,
                        position: Vec3::new(0.0, 0.0, 0.0),
                    },
                    GateConfig {
                        gate_type: GateType::Multiply(3),
                        movement: GateMovement::Static,
                        width: 5.0,
                        position: Vec3::new(0.0, 0.0, -9.0),
                    },
                ],
            }
        }
    }
}
