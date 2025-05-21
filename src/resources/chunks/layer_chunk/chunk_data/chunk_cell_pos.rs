use bevy::math::IVec3;

use crate::resources::chunks::{CHUNK_AREA, CHUNK_SIZE};

pub struct ChunkCellPos(usize);
impl ChunkCellPos {
    #[inline]
    pub fn idx_from_xyz(x: usize, y: usize, z: usize) -> usize {
        x + y * CHUNK_SIZE + z * CHUNK_AREA
    }

    #[inline]
    pub fn idx(&self) -> usize {
        self.0
    }

    pub fn new(idx: usize) -> Self {
        Self(idx)
    }

    pub fn from_xyz(x: usize, y: usize, z: usize) -> Self {
        Self(Self::idx_from_xyz(x, y, z))
    }

    #[inline]
    pub fn x(&self) -> usize {
        self.0 % CHUNK_SIZE
    }

    #[inline]
    pub fn y(&self) -> usize {
        (self.0 % CHUNK_AREA) / CHUNK_SIZE
    }

    #[inline]
    pub fn z(&self) -> usize {
        self.0 / CHUNK_AREA
    }

    /// Devuelve las 3 coordenadas juntas
    #[inline]
    pub fn to_xyz(&self) -> (usize, usize, usize) {
        (self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn to_ivec3(&self) -> IVec3 {
        IVec3 {
            x: self.x() as i32,
            y: self.y() as i32,
            z: self.z() as i32,
        }
    }
}
