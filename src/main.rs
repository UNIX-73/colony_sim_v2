pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins));

    app.run();
}
