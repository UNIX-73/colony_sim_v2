pub mod chunk_pos;
pub mod layer_chunk;
pub mod layers_data;

use crate::utils::{
    memory_size::MemorySize,
    multithread::{mutex::Mtx, rw_lock::Rw},
};
use bevy::prelude::*;
use chunk_pos::ChunkPos;
use layer_chunk::{
    LayerChunk,
    chunk_data::{ChunkData, chunk_cell_pos::ChunkCellPos},
    rle_layer::RleChunk,
};
use layers_data::{CellData, block::Block};
use once_cell::sync::Lazy;
use rand::Rng;
use std::{collections::HashMap, marker::PhantomData};
use strum::EnumCount;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_HEIGHT: usize = 60;

pub const CHUNK_AREA: usize = CHUNK_SIZE.pow(2);
pub const CHUNK_VOLUME: usize = CHUNK_AREA * CHUNK_HEIGHT;

pub static WORLD_CHUNKS: Lazy<Mtx<WorldChunks>> = Lazy::new(|| Mtx::new(WorldChunks::new()));

#[derive(Resource)]
pub struct WorldChunks {
    pub blocks: LayerChunks<Block, RleChunk<Block>>,
}
impl WorldChunks {
    pub fn new() -> WorldChunks {
        WorldChunks {
            blocks: LayerChunks::new(HashMap::new()),
        }
    }

    pub fn testing_world() -> WorldChunks {
        let chunk_radius = 20;
        let mut blocks_layer = LayerChunks::new(HashMap::new());

        for chunk_x in -chunk_radius..=chunk_radius {
            for chunk_y in -chunk_radius..=chunk_radius {
                let mut data = [Block::Air; CHUNK_VOLUME];
                let mut idx = 0_usize;
                for cell in &mut data {
                    if ChunkCellPos::new(idx).z() > 20 {
                        break;
                    }

                    let range = rand::rng().random_range(0..Block::COUNT as u16);

                    *cell = unsafe { std::mem::transmute::<u16, Block>(range) };

                    idx += 1;
                }

                blocks_layer.set_chunk(
                    ChunkPos {
                        x: chunk_x,
                        y: chunk_y,
                    },
                    ChunkData::new(data),
                );
            }
        }

        info!(
            "Generated test world with {} chunks - Memory usage: {}",
            (chunk_radius * 2 + 1).pow(2),
            blocks_layer.memory_usage().formatted_string()
        );

        WorldChunks {
            blocks: blocks_layer,
        }
    }
}
impl Default for WorldChunks {
    fn default() -> Self {
        WorldChunks {
            blocks: LayerChunks::new(HashMap::new()),
        }
    }
}

pub struct LayerChunks<T: CellData, Resolver: LayerChunk<T>> {
    chunks: HashMap<ChunkPos, Rw<Resolver>>,
    __: PhantomData<T>,
}
impl<T: CellData, Resolver: LayerChunk<T>> LayerChunks<T, Resolver> {
    pub fn new(chunks: HashMap<ChunkPos, Rw<Resolver>>) -> Self {
        Self {
            chunks,
            __: PhantomData,
        }
    }

    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<Rw<Resolver>> {
        self.chunks.get(&chunk_pos).map(|entry| (*entry).clone())
    }

    pub fn unload_chunk(&mut self, chunk_pos: ChunkPos) {
        self.chunks.remove(&chunk_pos);
    }

    pub fn set_chunk(&mut self, chunk_pos: ChunkPos, chunk: ChunkData<T>) {
        self.chunks
            .insert(chunk_pos, Rw::new(Resolver::from_unzip(chunk)));
    }

    /// Devuelve la cantidad de memoria usada por los chunks de capa (GB, MB, Bytes).
    pub fn memory_usage(&self) -> MemorySize {
        let mut memory = MemorySize::new(0);

        for chunk in self.chunks.iter() {
            let chunk_mem = chunk.1.read(|layer| layer.memory_usage());

            memory = memory + chunk_mem;
        }

        memory
    }
}
