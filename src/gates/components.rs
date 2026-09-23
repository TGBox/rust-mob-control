use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    Multiply(u32),
    Add(u32),
    Sub(u32),
    Divide(u32),
}

impl GateType {
    pub fn label(&self) -> String {
        match self {
            GateType::Multiply(v) => format!("x{}", v),
            GateType::Add(v) => format!("+{}", v),
            GateType::Sub(v) => format!("-{}", v),
            GateType::Divide(v) => format!("/{}", v),
        }
    }

    pub fn is_buff(&self) -> bool {
        matches!(self, GateType::Multiply(_) | GateType::Add(_))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GateMovement {
    Static,
    Sine {
        amplitude: f32,
        speed: f32,
        center_x: f32,
        phase: f32,
    },
}

#[derive(Component)]
pub struct Gate {
    pub gate_type: GateType,
    pub width: f32,
    pub movement: GateMovement,
}

#[derive(Component)]
pub struct GateVisual;
