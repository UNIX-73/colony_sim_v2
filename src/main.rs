pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;
use plugins::{
    camera::CameraPlugin, chunks::ChunksPlugin, grid::GridPlugin, render::RenderPlugin,
    testing::TestingPlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        TestingPlugin,
        DefaultPlugins,
        ChunksPlugin,
        GridPlugin,
        CameraPlugin,
        RenderPlugin,
    ));

    app.run();
}
