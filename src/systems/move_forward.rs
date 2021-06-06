use bevy::{
    core::Time,
    prelude::{Query, Res, Transform},
};

use crate::components::MoveForward;

pub fn move_forward(mut query: Query<(&MoveForward, &mut Transform)>, time: Res<Time>) {
    for (move_forward, mut transform) in query.iter_mut() {
        let move_vector = transform.local_y() * move_forward.speed;
        transform.translation += move_vector * time.delta_seconds();
    }
}
