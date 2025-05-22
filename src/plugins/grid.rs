use bevy::prelude::*;

use crate::systems::grid::sync_grid_transform;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sync_grid_transform);
    }
}
