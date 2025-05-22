use bevy::prelude::*;

use crate::components::{
    camera::{CameraComponent, MAX_CAMERA_RENDER_AREA_X, MAX_CAMERA_RENDER_AREA_Y},
    chunk::blocks::ChunkBlockComponent,
    grid::GridPos,
};

pub fn render_blocks(
    camera_query: Query<(&CameraComponent, &mut GridPos)>,
    blocks_query: Query<(Entity, &mut GridPos), With<ChunkBlockComponent>>,
) {
    debug_assert!(MAX_CAMERA_RENDER_AREA_X % 2 != 0 && MAX_CAMERA_RENDER_AREA_Y % 2 != 0);

    let mut visible_layer = 0;
    let mut camera_pos;
    if let Ok((camera, pos)) = camera_query.single() {
        camera_pos = pos;
        visible_layer = camera.visible_layer;
    }

    for x_area_displacement in 0..MAX_CAMERA_RENDER_AREA_X {
        for y_area_displacement in 0..MAX_CAMERA_RENDER_AREA_Y {
            
        }
    }
}
