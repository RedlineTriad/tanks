use bevy::{
    math::{Quat, Vec2},
    prelude::{GlobalTransform, Query, Res, Transform},
};

use crate::components::{Barrel, MousePosition};

pub fn barrel_aim(
    mouse: Res<MousePosition>,
    mut query: Query<(&Barrel, &mut Transform, &GlobalTransform)>,
) {
    for (_barrel, mut transform, global_transform) in query.iter_mut() {
        let mut global_transform = *global_transform;
        global_transform.translation.z = 0.;
        let rel_pos = mouse.world
            - Vec2::new(
                global_transform.translation.x,
                global_transform.translation.y,
            );
        let rel_angle = -rel_pos.x.atan2(rel_pos.y);

        let barrel_rot_pos = transform.local_y();
        let barrel_rot = -barrel_rot_pos.x.atan2(barrel_rot_pos.y);
        let world_rot_pos = global_transform.local_y();
        let world_rot = -world_rot_pos.x.atan2(world_rot_pos.y);

        transform.rotation = Quat::from_rotation_z(rel_angle - world_rot + barrel_rot);
    }
}
