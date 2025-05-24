use bevy::prelude::*;

use crate::{
    components::{
        camera::CameraComponent,
        grid::{GridPos, offset::GridPosOffset},
    },
    utils::math::extract_decimals_32,
};

pub fn sync_grid_transform(
    mut query: Query<
        (&GridPos, Option<&GridPosOffset>, &mut Transform),
        (
            Without<CameraComponent>,
            Or<(Changed<GridPos>, Changed<GridPosOffset>)>,
        ),
    >,
    mut camera_query: Query<
        (&CameraComponent, &GridPos, &GridPosOffset, &mut Transform),
        (
            Or<(
                Changed<GridPos>,
                Changed<GridPosOffset>,
                Changed<CameraComponent>,
            )>,
        ),
    >,
) {
    for (grid, offset, mut transform) in &mut query {
        let translation_offset = match offset {
            Some(offset) => offset.to_transform_translation_offset(),
            None => Vec3::ZERO,
        };

        transform.translation = grid.to_transform_translation() + translation_offset;
    }

    // Sólo la camara (+-zoom  en z)
    if let Ok((camera, grid, offset, mut transform)) = camera_query.single_mut() {
        let (int, dec) = extract_decimals_32(camera.zoom as f32);

        let offset_with_zoom = GridPosOffset::new(offset.x, offset.y, offset.z + dec as f32 - 0.5);
        let grid_with_zoom = GridPos::new(grid.x, grid.y, grid.z + int);

        transform.translation = grid_with_zoom.to_transform_translation()
            + offset_with_zoom.to_transform_translation_offset();
    }
}
