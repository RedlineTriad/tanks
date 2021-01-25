use bevy::prelude::*;
use components::{Barrel, MousePosition, MouseState, Tank};
use systems::{barrel_aim, mouse_position, move_forward, shoot, tank_movement};
mod components;
mod systems;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Tanks".to_string(),
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<MousePosition>()
        .add_startup_system(setup.system())
        .add_system(mouse_position.system())
        .add_system(tank_movement.system())
        .add_system(barrel_aim.system())
        .add_system(move_forward.system())
        .add_system(shoot.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tank_beige = asset_server.load("sprites/Tanks/tankBeige.png");
    let barrel_beige = asset_server.load("sprites/Tanks/barrelBeige.png");

    let camera = Camera2dBundle::default();
    let e = commands.spawn(camera).current_entity().unwrap();

    commands.insert_resource(MouseState {
        cursor: Default::default(),
        camera: e,
    });

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(tank_beige.into()),
            ..Default::default()
        })
        .with(Tank {
            speed: 500.,
            turn_speed: 3.,
        })
        .with_children(|parent| {
            parent
                .spawn((Transform::default(), GlobalTransform::default(), Barrel {}))
                .with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(0.0, 15.0, 1.0)),
                        material: materials.add(barrel_beige.into()),
                        ..Default::default()
                    });
                });
        });
}
