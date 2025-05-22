pub mod fractional;

use bevy::prelude::*;

use crate::{
    resources::chunks::{
        CHUNK_HEIGHT, CHUNK_SIZE, chunk_pos::ChunkPos,
        layer_chunk::chunk_data::chunk_cell_pos::ChunkCellPos,
    },
    utils::math::{div_floor, mod_floor},
};

pub const GRID_CELL_SIZE: f64 = 1.0;

#[derive(Component, Hash, Eq, PartialEq, Clone, Copy, Debug)]
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
}
impl Default for GridPos {
    fn default() -> Self {
        GridPos { x: 0, y: 0, z: 0 }
    }
}
