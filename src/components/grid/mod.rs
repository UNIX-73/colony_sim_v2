pub mod offset;

use bevy::prelude::*;
use offset::GridPosOffset;
use std::fmt;

use crate::{
    resources::chunks::{
        CHUNK_HEIGHT, CHUNK_SIZE, chunk_pos::ChunkPos,
        layer_chunk::chunk_data::chunk_cell_pos::ChunkCellPos,
    },
    utils::math::{div_floor, mod_floor},
};

pub const GRID_CELL_SIZE: f64 = 1.0;

#[derive(Component, Hash, Eq, PartialEq, Clone, Copy, Debug)]
#[require(Transform)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl GridPos {
    pub fn new(x: i32, y: i32, z: i32) -> GridPos {
        GridPos { x, y, z }
    }

    pub fn from_chunk(chunk_pos: ChunkPos, cell_pos: ChunkCellPos) -> GridPos {
        let (cell_x, cell_y, cell_z) = cell_pos.to_xyz();

        GridPos {
            x: cell_x as i32 + chunk_pos.x * CHUNK_SIZE as i32,
            y: cell_y as i32 + chunk_pos.y * CHUNK_SIZE as i32,
            z: cell_z as i32,
        }
    }

    #[inline]
    pub fn get_chunk_pos(&self) -> ChunkPos {
        ChunkPos::new(
            div_floor(self.x, CHUNK_SIZE as i32),
            div_floor(self.y, CHUNK_SIZE as i32),
        )
    }

    #[inline]
    pub fn get_chunk_cell_pos(&self) -> ChunkCellPos {
        let local_x = mod_floor(self.x, CHUNK_SIZE as i32) as usize;
        let local_y = mod_floor(self.y, CHUNK_SIZE as i32) as usize;
        let local_z = mod_floor(self.z, CHUNK_HEIGHT as i32) as usize;
        ChunkCellPos::from_xyz(local_x, local_y, local_z)
    }

    #[inline]
    pub fn to_transform_translation(&self) -> Vec3 {
        Vec3 {
            x: self.x as f32 * GRID_CELL_SIZE as f32,
            y: -self.y as f32 * GRID_CELL_SIZE as f32,
            z: self.z as f32 * GRID_CELL_SIZE as f32,
        }
    }

    pub fn apply_movement(&mut self, offset: &mut GridPosOffset, movement: Vec3) {
        offset.x += movement.x as f64;
        offset.y += movement.y as f64;
        offset.z += movement.z as f64;

        let delta_x = offset.x.floor() as i32;
        let delta_y = offset.y.floor() as i32;
        let delta_z = offset.z.floor() as i32;

        if delta_x != 0 || delta_y != 0 || delta_z != 0 {
            self.x += delta_x;
            self.y += delta_y;
            self.z += delta_z;

            offset.x -= delta_x as f64;
            offset.y -= delta_y as f64;
            offset.z -= delta_z as f64;
        }
    }

    pub fn apply_movement_with_trigger<F>(
        &mut self,
        offset: &mut GridPosOffset,
        movement: Vec3,
        mut callback: F,
    ) where
        F: FnMut((i32, i32, i32), &mut GridPos, &mut GridPosOffset) -> bool,
    {
        offset.x += movement.x as f64;
        offset.y += movement.y as f64;
        offset.z += movement.z as f64;

        loop {
            let delta_x = offset.x.floor() as i32;
            let delta_y = offset.y.floor() as i32;
            let delta_z = offset.z.floor() as i32;

            if delta_x == 0 && delta_y == 0 && delta_z == 0 {
                break;
            }

            // Callback puede decidir continuar o cancelar movimiento
            let should_continue = callback((delta_x, delta_y, delta_z), self, offset);
            if !should_continue {
                break;
            }

            self.x += delta_x;
            self.y += delta_y;
            self.z += delta_z;

            offset.x -= delta_x as f64;
            offset.y -= delta_y as f64;
            offset.z -= delta_z as f64;
        }
    }
}
impl Default for GridPos {
    fn default() -> Self {
        GridPos { x: 0, y: 0, z: 0 }
    }
}
impl fmt::Display for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:0}, {:1}, {:2})", self.x, self.y, self.z)
    }
}
