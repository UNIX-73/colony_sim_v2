use bevy::prelude::*;

pub fn setup_lights(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 4000.0,
            ..Default::default()
        },
        Transform {
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ..Default::default()
        },
    ));
}
