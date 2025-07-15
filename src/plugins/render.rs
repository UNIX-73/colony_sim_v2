use bevy::prelude::*;

use crate::{
    resources::render::RenderCache,
    systems::render::{lights::setup_lights, render_blocks},
};

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RenderCache::new())
            .add_systems(Startup, setup_lights)
            .add_systems(Update, render_blocks);
    }
}
