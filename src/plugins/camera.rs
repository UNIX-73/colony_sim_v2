use bevy::prelude::*;

use crate::systems::camera::{move_camera, setup_camera};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera);
    }
}
