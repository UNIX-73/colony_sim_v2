use bevy::prelude::*;

use crate::{
    components::{chunk::blocks::ChunkBlockComponent, grid::GridPos},
    debug_println,
};

pub struct TestingPlugin;
impl Plugin for TestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, testing_block);
    }
}

pub fn testing_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    /*
        commands.spawn((
            ChunkBlockComponent,
            GridPos::new(0, 0, 0),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 50))),
            Transform::default(),
        ));

        debug_println!("Spawned Test");
    */
}
