pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;
use plugins::chunks::ChunksPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ChunksPlugin));

    app.run();
}
