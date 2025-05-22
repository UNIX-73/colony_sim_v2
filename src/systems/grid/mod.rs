use bevy::prelude::*;

use crate::components::{
    camera::CameraComponent,
    grid::{GridPos, offset::GridPosOffset},
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
        let mut translation_offset = offset.to_transform_translation_offset();
        translation_offset.z += camera.zoom;
        transform.translation = grid.to_transform_translation() + translation_offset;
    }
}
