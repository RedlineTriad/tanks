use bevy::{
    math::Vec3,
    prelude::{
        AssetServer, Assets, Changed, Commands, DespawnRecursiveExt, Entity, GlobalTransform,
        Query, Res, ResMut, SpriteBundle, Transform,
    },
    sprite::ColorMaterial,
};

use crate::components::{Health, Lifetime, Radius};

pub fn death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Health, Option<&Radius>, Option<&GlobalTransform>), Changed<Health>>,
) {
    for (entity, health, radius, transform) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn_recursive();

            if let Some(radius) = radius {
                if let Some(transform) = transform {
                    let explosion_texture = asset_server.load("sprites/Smoke/smokeOrange0.png");
                    commands
                        .spawn_bundle(SpriteBundle {
                            material: materials.add(explosion_texture.into()),
                            transform: Transform {
                                translation: transform.translation,
                                scale: Vec3::new(1., 1., 1.) * radius.radius / 25.,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Lifetime(radius.radius / 25.));
                }
            }
        }
    }
}
