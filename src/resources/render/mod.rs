pub mod render_bit_map;

use std::num::NonZero;

use super::chunks::chunk_pos::ChunkPos;
use crate::utils::multithread::rw_lock::Rw;
use bevy::prelude::*;
use lru::LruCache;
use once_cell::sync::Lazy;
use render_bit_map::RenderBitMap;

pub const RENDERED_BLOCKS_LRU_ITEMS: usize = 20;
pub static mut RENDER_BLOCKS_CACHE: Lazy<RenderCache> = Lazy::new(|| RenderCache::new());

#[derive(Resource)]
pub struct RenderCache {
    pub blocks_cache: Rw<LruCache<ChunkPos, RenderBitMap>>,
}
impl RenderCache {
    pub fn new() -> RenderCache {
        RenderCache {
            blocks_cache: Rw::new(LruCache::new(
                NonZero::new(RENDERED_BLOCKS_LRU_ITEMS).unwrap(),
            )),
        }
    }
}
