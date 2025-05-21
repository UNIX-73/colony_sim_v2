use crate::resources::chunks::{CHUNK_VOLUME, CellData};
use chunk_cell_pos::ChunkCellPos;
pub mod chunk_cell_pos;

/*
    Representa la data expandida sin comprimir en un array que guarda todos los bloques
*/
#[derive(Clone, Debug)]
pub struct ChunkData<T: CellData>([T; CHUNK_VOLUME]);
impl<T: CellData> ChunkData<T> {
    pub fn get_mut(&mut self) -> &mut [T; CHUNK_VOLUME] {
        &mut self.0
    }

    pub fn new(data: [T; CHUNK_VOLUME]) -> ChunkData<T> {
        Self(data)
    }

    pub fn get_pos(&self, pos: ChunkCellPos) -> &T {
        debug_assert!(pos.idx() < CHUNK_VOLUME);
        &self.0[pos.idx()]
    }

    pub fn get_pos_mut(&mut self, pos: ChunkCellPos) -> &T {
        debug_assert!(pos.idx() < CHUNK_VOLUME);
        &mut self.0[pos.idx()]
    }

    /// Obtiene una referencia a la celda por índice lineal
    #[inline]
    pub fn get_idx(&self, idx: usize) -> &T {
        debug_assert!(idx < CHUNK_VOLUME);
        &self.0[idx]
    }

    /// Obtiene una referencia mutable a la celda por índice lineal
    #[inline]
    pub fn get_idx_mut(&mut self, idx: usize) -> &mut T {
        debug_assert!(idx < CHUNK_VOLUME);
        &mut self.0[idx]
    }

    /// Devuelve una referencia al array completo
    #[inline]
    pub fn data(&self) -> &[T; CHUNK_VOLUME] {
        &self.0
    }

    /// Devuelve una referencia mutable al array completo
    #[inline]
    pub fn data_mut(&mut self) -> &mut [T; CHUNK_VOLUME] {
        &mut self.0
    }
}
