use bevy::prelude::*;
use std::collections::HashMap;

use super::constants::SPATIAL_CELL_SIZE;

#[derive(Resource, Default)]
pub struct SpatialHash2D {
    grid: HashMap<(i32, i32), Vec<(Entity, Vec2)>>,
    cell_size: f32,
}

impl SpatialHash2D {
    pub fn new() -> Self {
        Self {
            grid: HashMap::with_capacity(1024),
            cell_size: SPATIAL_CELL_SIZE,
        }
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    #[inline]
    fn to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, entity: Entity, pos: Vec2) {
        let cell = self.to_cell(pos);
        self.grid.entry(cell).or_default().push((entity, pos));
    }

    /// Finds all entities within `radius` of `pos`. Returns `(Entity, Vec2, dist_sq)`.
    pub fn query_radius(&self, pos: Vec2, radius: f32) -> Vec<(Entity, Vec2, f32)> {
        let mut results = Vec::with_capacity(16);
        let r_sq = radius * radius;
        let min_cell = self.to_cell(pos - Vec2::splat(radius));
        let max_cell = self.to_cell(pos + Vec2::splat(radius));

        for cx in min_cell.0..=max_cell.0 {
            for cy in min_cell.1..=max_cell.1 {
                if let Some(entities) = self.grid.get(&(cx, cy)) {
                    for &(entity, other_pos) in entities {
                        let dist_sq = pos.distance_squared(other_pos);
                        if dist_sq <= r_sq {
                            results.push((entity, other_pos, dist_sq));
                        }
                    }
                }
            }
        }
        results
    }
}
