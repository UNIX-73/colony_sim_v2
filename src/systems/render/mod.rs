pub mod block_instancing;
pub mod lights;

use crate::{
    components::{
        camera::{CameraComponent, MAX_CAMERA_RENDER_AREA_X, MAX_CAMERA_RENDER_AREA_Y},
        chunk::blocks::ChunkBlockComponent,
        grid::GridPos,
    },
    resources::{
        SharedThreadResource,
        chunks::{
            CHUNK_SIZE, WorldChunks,
            chunk_pos::ChunkPos,
            layer_chunk::{LayerChunk, chunk_data::chunk_cell_pos::ChunkCellPos},
            layers_data::{OcclussionCulling, block::material_resource::BlockInstancing},
        },
        render::{RENDERED_BLOCKS_LRU_ITEMS, RenderCache, render_bit_map::RenderBitMap},
    },
};
use bevy::prelude::*;

pub fn render_blocks(
    mut commands: Commands,
    block_instancing: Res<BlockInstancing>,
    chunks: Res<SharedThreadResource<WorldChunks>>,
    rendered_cache: Res<RenderCache>,
    camera_query: Query<(&CameraComponent, &GridPos)>,
    blocks_query: Query<(Entity, &GridPos), With<ChunkBlockComponent>>,
) {
    debug_assert!(MAX_CAMERA_RENDER_AREA_X % 2 != 0 && MAX_CAMERA_RENDER_AREA_Y % 2 != 0);

    let (camera, camera_pos) = match camera_query.single() {
        Ok(res) => res,
        Err(_) => return,
    };

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
            if let Some(rw_rle) = chunks.get().read(|world| world.blocks.get_chunk(chunk_pos)) {
                let blocks = rw_rle.read(|rle| rle.unzip());

                let chunk_origin_x = chunk_pos.x * CHUNK_SIZE as i32;
                let chunk_origin_y = chunk_pos.y * CHUNK_SIZE as i32;

                let mut rendered_this_loop = RenderBitMap::default();
                let prerendered_cache = rendered_cache
                    .blocks_cache
                    .write(|lru| lru.get(&chunk_pos).cloned())
                    .unwrap_or(RenderBitMap::default());

                for world_z in 0..=camera.visible_layer as i32 {
                    for local_y in 0..CHUNK_SIZE as i32 {
                        for local_x in 0..CHUNK_SIZE as i32 {
                            let world_x = chunk_origin_x + local_x;
                            let world_y = chunk_origin_y + local_y;

                            // Solo renderizar si está dentro del área visible
                            if world_x < render_min_x
                                || world_x > render_max_x
                                || world_y < render_min_y
                                || world_y > render_max_y
                            {
                                continue;
                            }

                            let index = ChunkCellPos::from_xyz(
                                local_x as usize,
                                local_y as usize,
                                world_z as usize,
                            );
                            rendered_this_loop.set_cell(index, true);

                            if prerendered_cache.get_cell(index) {
                                continue;
                            }

                            let block = blocks.get_pos(index);

                            if !block.occludes()
                                || blocks.is_occluded_with_visible_layer(
                                    index,
                                    camera.visible_layer as usize,
                                )
                            {
                                continue;
                            }

                            if let Some((material, mesh)) = block_instancing.get_components(*block)
                            {
                                commands.spawn((
                                    Mesh3d(mesh),
                                    MeshMaterial3d(material),
                                    ChunkBlockComponent,
                                    GridPos::new(world_x, world_y, world_z),
                                    Transform::default(),
                                ));
                            }
                        }
                    }
                }

                rendered_cache.blocks_cache.write(|lru| {
                    let bit_map = lru.get_mut(&chunk_pos);
                    if let Some(bm) = bit_map {
                        *bm = rendered_this_loop;
                    } else {
                        lru.push(chunk_pos, rendered_this_loop);
                    }
                });
            }
        }
    }

    rendered_cache.blocks_cache.read(|lru| {
        let mut rendered_bm: [Option<(ChunkPos, &RenderBitMap)>; RENDERED_BLOCKS_LRU_ITEMS] =
            [None; RENDERED_BLOCKS_LRU_ITEMS];

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
}
