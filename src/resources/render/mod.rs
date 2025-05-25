pub mod render_bit_map;

use std::collections::HashSet;
use bevy::prelude::*;
use crate::components::grid::GridPos;

#[derive(Resource, Default)]
pub struct RenderCache {
    pub rendered_blocks: HashSet<GridPos>,
}
impl RenderCache {
    pub fn new() -> RenderCache {
        RenderCache {
            rendered_blocks: HashSet::new(),
        }
    }
}
