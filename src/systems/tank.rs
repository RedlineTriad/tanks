use bevy::{
    core::Time,
    ecs::{Query, Res},
    input::Input,
    math::{Quat, Vec3},
    prelude::{KeyCode, Transform},
};

use crate::components::Tank;

pub fn tank_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Tank, &mut Transform)>,
) {
    for (tank, mut transform) in query.iter_mut() {
        let mut turn_input = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            turn_input -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            turn_input += 1.0;
        }

        let mut drive = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            drive += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            drive -= 1.0
        }

        transform.rotate(Quat::from_rotation_z(
            -turn_input * tank.turn_speed * time.delta_seconds(),
        ));
        let move_dir =
            transform.rotation * Vec3::unit_y() * drive * tank.speed * time.delta_seconds();
        transform.translation += move_dir;
    }
}
