use bevy::prelude::*;

pub const MAX_CAMERA_Z: u32 = 80;

#[derive(Component)]
pub struct CameraComponent {
    pub speed: f32,
    pub visible_layer: u32,
    pub zoom: f32,
}
