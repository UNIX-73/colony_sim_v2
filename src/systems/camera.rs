use bevy::prelude::*;

use crate::components::{
    camera::{CameraComponent, MAX_CAMERA_Z},
    grid::{GridPos, offset::GridPosOffset},
};

const MIN_ZOOM: f64 = 1.0;
const MAX_ZOOM: f64 = 100.0;

const CAMERA_MIN_POLE_LENGTH: u32 = 10;
const CAMERA_START_ZOOM: f64 = 10.0;
const CAMERA_BASE_SPEED: f64 = 15.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        CameraComponent {
            speed: CAMERA_BASE_SPEED,
            visible_layer: 0,
            zoom: CAMERA_START_ZOOM,
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::ZERO, -Vec3::X)
            .with_rotation(Quat::from_xyzw(
                -f32::sqrt(2.0) / 2.0, // x
                0.0,                   // y
                0.0,                   // z
                f32::sqrt(2.0) / 2.0,  // w
            )),
        GridPos::new(0, 0, CAMERA_MIN_POLE_LENGTH as i32),
        GridPosOffset::default(),
    ));

    println!("Spawned camera");
}

pub fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut CameraComponent, &mut GridPos, &mut GridPosOffset)>,
) {
    if let Ok((mut camera, mut grid, mut offset)) = camera_query.single_mut() {
        // Movmimiento xy
        let mut dir = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) {
            dir.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            dir.y += 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            dir.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            dir.x += 1.0;
        }
        if dir != Vec2::ZERO {
            dir = dir.normalize();
            let movement = dir * camera.speed as f32 * time.delta_secs();

            grid.apply_movement_2d(&mut offset, movement);
        }

        let mut changed_layer = true;
        match (
            input.just_pressed(KeyCode::PageUp),
            input.just_pressed(KeyCode::PageDown),
        ) {
            (true, false) if camera.visible_layer < MAX_CAMERA_Z => {
                camera.visible_layer += 1;
                grid.z += 1;
            }
            (false, true) if camera.visible_layer > 0 => {
                camera.visible_layer -= 1;
                grid.z -= 1;
            }
            _ => changed_layer = false,
        }
        if changed_layer {
            println!("Camera layer {}", camera.visible_layer);
        }

        let mut changed_zoom = true;
        match (input.pressed(KeyCode::Home), input.pressed(KeyCode::End)) {
            (true, false) => camera.zoom += time.delta_secs_f64() * camera.speed,
            (false, true) => camera.zoom -= time.delta_secs_f64() * camera.speed,
            _ => changed_zoom = false,
        }
        if changed_zoom {
            camera.zoom = camera.zoom.clamp(MIN_ZOOM, MAX_ZOOM);
            println!("Camera zoom {}", camera.zoom);
        }
    }
}
