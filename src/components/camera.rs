use bevy::prelude::*;

pub const MAX_CAMERA_Z: u32 = 80;

pub const MAX_CAMERA_RENDER_AREA_X: usize = 5;
pub const MAX_CAMERA_RENDER_AREA_Y: usize = 3;

#[derive(Component)]
pub struct CameraComponent {
    pub speed: f32,
    pub visible_layer: u32,
    pub zoom: f32,
}
