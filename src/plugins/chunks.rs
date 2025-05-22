use bevy::app::Plugin;

use crate::resources::chunks::WorldChunks;

pub struct ChunksPlugin;
impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(WorldChunks::testing_world());
    }
}
