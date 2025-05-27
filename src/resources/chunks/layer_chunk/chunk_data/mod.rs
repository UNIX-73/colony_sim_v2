use crate::resources::chunks::{CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_VOLUME, CellData};
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

    /// Devuelve un bool que representa si el bloque está bloqueado por todos sus lados
    #[inline]
    pub fn is_occluded(&self, pos: ChunkCellPos) -> bool {
        for (dx, dy, dz) in OCCLUSION_DIRECTIONS {
            let displaced = pos.get_displaced(dx, dy, dz);

            if let Some(displaced) = displaced {
                let block = self.get_pos(displaced);

                if !block.occludes() {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    #[inline]
    pub fn is_occluded_with_visible_layer(&self, pos: ChunkCellPos, visible_layer: usize) -> bool {
        if pos.z() >= visible_layer {
            return false;
        }

        self.is_occluded(pos)
    }
}
const OCCLUSION_DIRECTIONS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];
