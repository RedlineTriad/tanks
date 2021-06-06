use bevy::{
    input::Input,
    prelude::{
        AssetServer, Assets, Commands, GlobalTransform, MouseButton, Query, Res, ResMut,
        SpriteBundle, Transform,
    },
    sprite::ColorMaterial,
};

use crate::components::{Barrel, Health, HealthEffect, MoveForward, Radius, Team};

pub fn shoot(
    mouse_button_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Barrel, &Team, &GlobalTransform)>,
) {
    for (_, team, global_transform) in query.iter() {
        if mouse_button_input.just_released(MouseButton::Left) {
            let bullet_texture = asset_server
                .load(&format!("sprites/Bullets/bullet{}.png", team.color().name())[..]);
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(bullet_texture.into()),
                    transform: Transform {
                        translation: global_transform.translation,
                        rotation: global_transform.rotation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*team)
                .insert(HealthEffect { amount: -100 })
                .insert(Health(1))
                .insert(Radius { radius: 10. })
                .insert(MoveForward { speed: 2000. });
        }
    }
}
