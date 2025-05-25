use crate::resources::chunks::{
    CHUNK_VOLUME, layer_chunk::chunk_data::chunk_cell_pos::ChunkCellPos,
};

const U64_BITS: usize = u64::BITS as usize;
const BIT_MAP_SIZE: usize = CHUNK_VOLUME / U64_BITS;

#[derive(Clone)]
pub struct RenderBitMap([u64; BIT_MAP_SIZE]);
impl Default for RenderBitMap {
    fn default() -> Self {
        Self([0b0; BIT_MAP_SIZE])
    }
}
impl RenderBitMap {
    pub fn get_mut(&mut self) -> &mut [u64; BIT_MAP_SIZE] {
        &mut self.0
    }

    pub fn get_cell(&self, cell_pos: ChunkCellPos) -> bool {
        let word_idx = cell_pos.idx() / U64_BITS;
        let idx = cell_pos.idx() % U64_BITS;

        let word = self.0[word_idx];

        word & (0b1 << idx) == 0b1
    }

    pub fn set_cell(&mut self, cell_pos: ChunkCellPos, value: bool) {
        let word_idx = cell_pos.idx() / U64_BITS;
        let idx = cell_pos.idx() % U64_BITS;

        let word = &mut self.0[word_idx];

        if value {
            *word |= 1 << idx;
        } else {
            *word &= !(1 << idx);
        }
    }
}
