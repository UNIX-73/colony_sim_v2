use bevy::prelude::*;

use crate::resources::chunks::layers_data::block::material_resource::BlockInstancing;

pub fn init_block_instancing(
    mut commands: Commands,
    materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(BlockInstancing::init(materials, meshes));
}
