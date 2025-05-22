use bevy::prelude::*;

use crate::components::{
    camera::{CameraComponent, MAX_CAMERA_Z},
    grid::{GridPos, offset::GridPosOffset},
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        CameraComponent {
            speed: 35.0,
            visible_layer: 0,
            zoom: 20.0,
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 18.0, 0.0)
            .looking_at(Vec3::ZERO, -Vec3::X)
            .with_rotation(Quat::from_xyzw(
                -f32::sqrt(2.0) / 2.0, // x
                0.0,                   // y
                0.0,                   // z
                f32::sqrt(2.0) / 2.0,  // w
            )),
        GridPos::default(),
        GridPosOffset::default(),
    ));
}

pub fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut CameraComponent, &mut GridPos, &mut GridPosOffset)>,
) {
    if let Ok((mut camera, mut grid, mut offset)) = camera_query.single_mut() {
        println!("{}", grid.clone());

        // Movmimiento xy
        let mut dir = Vec2::ZERO;
        let delta_s = time.delta_secs();

        if input.pressed(KeyCode::KeyW) {
            dir.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            dir.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            dir.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            dir.x += 1.0;
        }

        if dir != Vec2::ZERO {
            dir = dir.normalize();
            let movement = dir * camera.speed * delta_s;

            grid.apply_movement(
                &mut offset,
                Vec3 {
                    x: movement.x,
                    y: movement.y,
                    z: 0.0,
                },
            );
        }

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
            _ => {}
        }

        match (
            input.just_pressed(KeyCode::Home),
            input.just_pressed(KeyCode::End),
        ) {
            (true, false) => camera.zoom += 1.0,
            (false, true) => camera.zoom -= 1.0,
            _ => (),
        }
        camera.zoom = camera.zoom.clamp(2.0, 40.0);
    }
}
