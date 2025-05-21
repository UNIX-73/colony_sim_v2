pub mod chunk_data;
pub mod rle_layer;

use super::CellData;
use crate::utils::memory_size::MemorySize;
use chunk_data::{ChunkData, chunk_cell_pos::ChunkCellPos};

pub trait LayerChunk<T: CellData> {
    fn get_index(&self, idx: usize) -> T;
    fn get_pos(&self, pos: ChunkCellPos) -> T;
    fn get_xyz(&self, x: usize, y: usize, z: usize) -> T;

    /// Descomprime a un array plano de T.
    fn unzip(&self) -> ChunkData<T>;

    /// Comprime a el método de almacenamiento de T
    fn from_unzip(unzip: ChunkData<T>) -> Self;

    fn memory_usage(&self) -> MemorySize;

    /// Devuelve true si la capa es válida internamente (útil para compresión).
    fn is_valid(&self) -> bool;

    fn iter(&self) -> impl Iterator<Item = (usize, T)> + '_;
}
