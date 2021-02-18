use bevy::{
    core::Time,
    ecs::{Commands, Entity, Query, Res},
    prelude::DespawnRecursiveExt,
};

use crate::components::Lifetime;

pub fn lifetime(mut query: Query<&mut Lifetime>, time: Res<Time>) {
    for mut lifetime in query.iter_mut() {
        lifetime.0 -= time.delta_seconds();
    }
}

pub fn lifetime_destroy(query: Query<(&Lifetime, Entity)>, commands: &mut Commands) {
    for (Lifetime(lifetime), entity) in query.iter() {
        if *lifetime < 0. {
            commands.despawn_recursive(entity);
        }
    }
}
