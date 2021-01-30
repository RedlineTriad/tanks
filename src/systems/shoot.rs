use bevy::{
    ecs::{Commands, Query, Res, ResMut},
    input::Input,
    prelude::{AssetServer, Assets, GlobalTransform, MouseButton, SpriteBundle, Transform},
    sprite::ColorMaterial,
};

use crate::components::{Barrel, HealthEffect, MoveForward, Radius, Team};

pub fn shoot(
    mouse_button_input: Res<Input<MouseButton>>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Barrel, &Team, &GlobalTransform)>,
) {
    for (_, team, global_transform) in query.iter() {
        if mouse_button_input.just_released(MouseButton::Left) {
            let bullet_texture = asset_server.load("sprites/Bullets/bulletBeige.png");
            commands
                .spawn(SpriteBundle {
                    material: materials.add(bullet_texture.into()),
                    transform: Transform {
                        translation: global_transform.translation,
                        rotation: global_transform.rotation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with((*team).clone())
                .with(HealthEffect { amount: -100 })
                .with(Radius { radius: 10. })
                .with(MoveForward { speed: 2000. });
        }
    }
}
