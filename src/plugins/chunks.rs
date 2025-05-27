use bevy::{
    app::{Plugin, Startup},
};

use crate::{
    resources::{
        SharedThreadResource,
        chunks::{WORLD_CHUNKS, WorldChunks},
    },
    systems::render::block_instancing::init_block_instancing,
    utils::multithread::mutex::Mtx,
};

pub struct ChunksPlugin;
impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let chunks: Mtx<WorldChunks> = WORLD_CHUNKS.clone();

        chunks.write(|world| *world = WorldChunks::testing_world());

        app.insert_resource(SharedThreadResource::new(WORLD_CHUNKS.clone()))
            .add_systems(Startup, init_block_instancing);
    }
}
