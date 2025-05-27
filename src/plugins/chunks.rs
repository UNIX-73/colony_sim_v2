use bevy::app::{Plugin, Startup};

use crate::{
    resources::chunks::WorldChunks, systems::render::block_instancing::init_block_instancing,
};

pub struct ChunksPlugin;
impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(WorldChunks::testing_world())
            .add_systems(Startup, init_block_instancing);
    }
}
