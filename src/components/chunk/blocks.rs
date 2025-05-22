use bevy::prelude::*;

use crate::components::grid::GridPos;

#[derive(Component)]
#[require(Transform, GridPos)]
pub struct ChunkBlockComponent;
