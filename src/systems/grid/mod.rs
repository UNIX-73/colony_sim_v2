use bevy::prelude::*;

use crate::components::grid::{GridPos, offset::GridPosOffset};

pub fn sync_grid_transform(
    mut query: Query<
        (&GridPos, Option<&GridPosOffset>, &mut Transform),
        Or<(Changed<GridPos>, Changed<GridPosOffset>)>,
    >,
) {
    for (grid, offset, mut transform) in &mut query {
        let translation_offset = match offset {
            Some(offset) => offset.to_transform_translation_offset(),
            None => Vec3::ZERO,
        };

        transform.translation = grid.to_transform_translation() + translation_offset;
    }
}
