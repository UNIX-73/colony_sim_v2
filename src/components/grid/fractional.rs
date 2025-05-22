use super::{GRID_CELL_SIZE, GridPos};
use bevy::prelude::*;

#[derive(Component, PartialEq, Clone, Copy, Debug)]
#[require(GridPos)]
pub struct GridPosOffset {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GridPosOffset {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let mut new = Self { x: x, y: y, z: z };
        new.clamp();

        new
    }

    pub fn clamp(&mut self) {
        self.x = self.x.clamp(-0.5, 0.5);
        self.y = self.y.clamp(-0.5, 0.5);
        self.z = self.z.clamp(-0.5, 0.5);
    }

    /// Lleva el valor al rango [-0.5, 0.5]
    fn wrap(value: f64) -> f64 {
        let mut wrapped = value % 1.0;
        if wrapped >= 0.5 {
            wrapped -= 1.0;
        } else if wrapped < -0.5 {
            wrapped += 1.0;
        }
        wrapped
    }

    pub fn to_transform_translation_offset(&self) -> Vec3 {
        Vec3 {
            x: self.x as f32 * GRID_CELL_SIZE as f32,
            y: -self.y as f32 * GRID_CELL_SIZE as f32,
            z: self.z as f32 * GRID_CELL_SIZE as f32,
        }
    }
}
