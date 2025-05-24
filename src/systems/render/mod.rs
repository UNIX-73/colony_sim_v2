use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    components::{
        camera::{CameraComponent, MAX_CAMERA_RENDER_AREA_X, MAX_CAMERA_RENDER_AREA_Y},
        chunk::blocks::ChunkBlockComponent,
        grid::GridPos,
    },
    debug_perf,
    resources::{
        chunks::{
            CHUNK_SIZE, WorldChunks,
            chunk_pos::ChunkPos,
            layer_chunk::{LayerChunk, chunk_data::chunk_cell_pos::ChunkCellPos},
            layers_data::block::SurfaceBlock,
        },
        render::RenderCache,
    },
};

pub fn render_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunks: Res<WorldChunks>,
    mut rendered_cache: ResMut<RenderCache>,
    camera_query: Query<(&CameraComponent, &GridPos)>,
    blocks_query: Query<(Entity, &GridPos), With<ChunkBlockComponent>>,
) {
    debug_assert!(MAX_CAMERA_RENDER_AREA_X % 2 != 0 && MAX_CAMERA_RENDER_AREA_Y % 2 != 0);

    let mut rendered: HashSet<GridPos> = HashSet::new();

    let (camera, camera_pos) = match camera_query.single() {
        Ok(res) => res,
        Err(_) => return,
    };

    debug_perf!("RENDER", {
        let camera_chunk = camera_pos.get_chunk_pos();
        let camera_chunk_cell_pos = camera_pos.get_chunk_cell_pos();

        let half_area_x = MAX_CAMERA_RENDER_AREA_X as i32 / 2;
        let half_area_y = MAX_CAMERA_RENDER_AREA_Y as i32 / 2;

        let plus_x = camera_chunk_cell_pos.x() as i32 + half_area_x;
        let plus_y = camera_chunk_cell_pos.y() as i32 + half_area_y;
        let minus_x = camera_chunk_cell_pos.x() as i32 - half_area_x;
        let minus_y = camera_chunk_cell_pos.y() as i32 - half_area_y;

        let chunk_plus_x = plus_x.div_euclid(CHUNK_SIZE as i32);
        let chunk_plus_y = plus_y.div_euclid(CHUNK_SIZE as i32);
        let chunk_minus_x = minus_x.div_euclid(CHUNK_SIZE as i32);
        let chunk_minus_y = minus_y.div_euclid(CHUNK_SIZE as i32);

        let camera_world_x = camera_pos.x;
        let camera_world_y = camera_pos.y;

        let render_min_x = camera_world_x - half_area_x;
        let render_max_x = camera_world_x + half_area_x;
        let render_min_y = camera_world_y - half_area_y;
        let render_max_y = camera_world_y + half_area_y;

        // Chunk
        for chunk_y in chunk_minus_y..=chunk_plus_y {
            for chunk_x in chunk_minus_x..=chunk_plus_x {
                let chunk_pos = ChunkPos::new(camera_chunk.x + chunk_x, camera_chunk.y + chunk_y);

                if let Some(rle) = chunks.blocks.get_chunk(chunk_pos) {
                    let blocks = rle.unzip();

                    let chunk_origin_x = chunk_pos.x * CHUNK_SIZE as i32;
                    let chunk_origin_y = chunk_pos.y * CHUNK_SIZE as i32;

                    // Cells
                    for world_z in 0..=camera.visible_layer as i32 {
                        for local_y in 0..CHUNK_SIZE as i32 {
                            for local_x in 0..CHUNK_SIZE as i32 {
                                let world_x = chunk_origin_x + local_x;
                                let world_y = chunk_origin_y + local_y;

                                // Solo renderizar si está dentro del área visible
                                if world_x >= render_min_x
                                    && world_x <= render_max_x
                                    && world_y >= render_min_y
                                    && world_y <= render_max_y
                                {
                                    let index = ChunkCellPos::from_xyz(
                                        local_x as usize,
                                        local_y as usize,
                                        world_z as usize,
                                    );

                                    let grid_pos = index.to_grid_pos(chunk_pos);

                                    rendered.insert(grid_pos);
                                    if rendered_cache.rendered_blocks.contains(&grid_pos) {
                                        continue;
                                    }

                                    let block = blocks.get_pos(index);

                                    if *block == SurfaceBlock::Air {
                                        continue;
                                    }
                                    let color = block.get_color().unwrap_or((0, 0, 0));

                                    let pos = GridPos::new(world_x, world_y, world_z);

                                    commands.spawn((
                                        ChunkBlockComponent,
                                        pos,
                                        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                                        MeshMaterial3d(
                                            materials
                                                .add(Color::srgb_u8(color.0, color.1, color.2)),
                                        ),
                                        Transform::default(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        for (entity, pos) in &blocks_query {
            if !rendered.contains(pos) {
                commands.entity(entity).despawn();
            }
        }

        // Actualizamos el cache de bloques renderizados
        if !rendered.is_empty() {
            rendered_cache.rendered_blocks = rendered;
        }
    });
}
