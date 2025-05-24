use bevy::prelude::*;

use crate::{resources::render::RenderCache, systems::render::render_blocks};

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RenderCache::new())
            .add_systems(Update, render_blocks);
    }
}
