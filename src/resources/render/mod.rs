pub mod render_bit_map;

use std::num::NonZero;

use crate::utils::rw_lock::Rw;
use bevy::prelude::*;
use lru::LruCache;
use render_bit_map::RenderBitMap;

use super::chunks::chunk_pos::ChunkPos;

const LRU_ITEMS: usize = 10;

#[derive(Resource)]
pub struct RenderCache {
    pub blocks_cache: Rw<LruCache<ChunkPos, RenderBitMap>>,
}
impl RenderCache {
    pub fn new() -> RenderCache {
        RenderCache {
            blocks_cache: Rw::new(LruCache::new(NonZero::new(LRU_ITEMS).unwrap())),
        }
    }
}
