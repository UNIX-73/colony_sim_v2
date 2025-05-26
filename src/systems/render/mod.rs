use std::{array, collections::HashMap};

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
        render::{RENDERED_BLOCKS_LRU_ITEMS, RenderCache, render_bit_map::RenderBitMap},
    },
};

pub fn render_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunks: Res<WorldChunks>,
    rendered_cache: Res<RenderCache>,
    camera_query: Query<(&CameraComponent, &GridPos)>,
    blocks_query: Query<(Entity, &GridPos), With<ChunkBlockComponent>>,
) {
    debug_assert!(MAX_CAMERA_RENDER_AREA_X % 2 != 0 && MAX_CAMERA_RENDER_AREA_Y % 2 != 0);

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

                // Cells
                if let Some(rw_rle) = chunks.blocks.get_chunk(chunk_pos) {
                    let blocks = rw_rle.read(|rle| rle.unzip());

                    let chunk_origin_x = chunk_pos.x * CHUNK_SIZE as i32;
                    let chunk_origin_y = chunk_pos.y * CHUNK_SIZE as i32;

                    let mut prerendered_cache = rendered_cache
                        .blocks_cache
                        .write(|lru| lru.get(&chunk_pos).cloned())
                        .unwrap_or(RenderBitMap::default());

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

                                    // FIXME: La mayor parte de mal rendimiento viene del contains ya que no tiene buena cache y se llama millones de veces / s
                                    // Intentar usar arrays con bitmaps
                                    if prerendered_cache.get_cell(index) {
                                        continue;
                                    }
                                    prerendered_cache.set_cell(index, true);

                                    let block = blocks.get_pos(index);

                                    if *block == SurfaceBlock::Air {
                                        continue;
                                    }
                                    let color = block.get_color().unwrap_or((0, 0, 0));

                                    commands.spawn((
                                        ChunkBlockComponent,
                                        GridPos::new(world_x, world_y, world_z),
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

                    rendered_cache.blocks_cache.write(|lru| {
                        let bit_map = lru.get_mut(&chunk_pos);
                        if let Some(bm) = bit_map {
                            *bm = prerendered_cache;
                        } else {
                            lru.push(chunk_pos, prerendered_cache);
                        }
                    });
                }
            }
        }

        debug_perf!("DESPAWN", {
            rendered_cache.blocks_cache.read(|lru| {
                let mut rendered_bm: [Option<(ChunkPos, &RenderBitMap)>;
                    RENDERED_BLOCKS_LRU_ITEMS] = [None; RENDERED_BLOCKS_LRU_ITEMS];

                // Pasamos al stack para mejorar rendimiento
                let mut i = 0_usize;
                for (pos, bm) in lru.iter() {
                    rendered_bm[i] = Some((pos.clone(), bm));
                    i += 1;
                }

                // Se elimina  a no ser que bm.get_cell sea true, en ese caso como si esta renderizado se pasa a despawn = false;
                let mut despawn;
                for (entity, pos) in &blocks_query {
                    despawn = true;

                    let chunk_pos = pos.get_chunk_pos();
                    let cell_pos = pos.get_chunk_cell_pos();

                    for data in &rendered_bm {
                        if let Some((c_pos, bm)) = data {
                            if *c_pos == chunk_pos {
                                despawn = !bm.get_cell(cell_pos);
                                break;
                            }
                            continue;
                        } else {
                            break;
                        }
                    }

                    if despawn {
                        commands.entity(entity).despawn();
                    }
                }
            });
        });
    });
}
