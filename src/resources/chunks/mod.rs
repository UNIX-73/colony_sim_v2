pub mod chunk_pos;
pub mod layer_chunk;
pub mod layers_data;

use crate::utils::memory_size::MemorySize;
use bevy::prelude::*;
use chunk_pos::ChunkPos;
use layer_chunk::{LayerChunk, chunk_data::ChunkData};
use std::{collections::HashMap, marker::PhantomData};

pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_HEIGHT: usize = 60;

pub const CHUNK_AREA: usize = CHUNK_SIZE.pow(2);
pub const CHUNK_VOLUME: usize = CHUNK_AREA * CHUNK_HEIGHT;

pub trait CellData: Clone + PartialEq + Default {}
impl<T: Clone + PartialEq + Default> CellData for T {}

#[derive(Resource)]
pub struct WorldChunks {}
impl WorldChunks {}

pub struct LayerChunks<T: CellData, Chunk: LayerChunk<T>> {
    chunks: HashMap<ChunkPos, Chunk>,
    __: PhantomData<T>,
}
impl<T: CellData, Chunk: LayerChunk<T>> LayerChunks<T, Chunk> {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            __: PhantomData,
        }
    }

    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&chunk_pos)
    }

    pub fn get_chunk_mut(&mut self, chunk_pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&chunk_pos)
    }

    pub fn unload_chunk(&mut self, chunk_pos: ChunkPos) {
        self.chunks.remove(&chunk_pos);
    }

    pub fn set_chunk(&mut self, chunk_pos: ChunkPos, chunk: ChunkData<T>) {
        self.chunks.insert(chunk_pos, Chunk::from_unzip(chunk));
    }

    /// Devuelve la cantidad de memoria usada por los chunks de capa (GB, MB, Bytes).
    pub fn memory_usage(&self) -> MemorySize {
        let mut memory = MemorySize::new(0);

        for (_, chunk) in self.chunks.iter() {
            let chunk_mem = chunk.memory_usage();

            memory = memory + chunk_mem;
        }

        memory
    }
}
