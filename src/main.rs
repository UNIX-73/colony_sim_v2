pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;
use plugins::{camera::CameraPlugin, chunks::ChunksPlugin, grid::GridPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ChunksPlugin, GridPlugin, CameraPlugin));

    app.run();
}
