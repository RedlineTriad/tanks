use bevy::prelude::*;
use components::{Barrel, MousePosition, MouseState, Radius, Tank, Team};
use systems::{barrel_aim, mouse_position, move_forward, shoot, tank_hit, tank_movement};
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
        .add_system(tank_hit.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    let camera = Camera2dBundle::default();
    let e = commands.spawn(camera).current_entity().unwrap();

    commands.insert_resource(MouseState {
        cursor: Default::default(),
        camera: e,
    });

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    for x in -5..5 {
        for y in -5..5 {
            println!("Spawning at {} {}", x, y);
            create_tank(
                commands,
                &mut materials,
                &mut asset_server,
                Transform {
                    translation: Vec3::new(x as f32 * 200., y as f32 * 200., 0.),
                    ..Default::default()
                },
                x + y,
            );
        }
    }
}

fn create_tank(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &mut Res<AssetServer>,
    transform: Transform,
    team: i64,
) {
    let tank_beige = asset_server.load("sprites/Tanks/tankBeige.png");
    let barrel_beige = asset_server.load("sprites/Tanks/barrelBeige.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(tank_beige.into()),
            transform,
            ..Default::default()
        })
        .with(Tank {
            speed: 500.,
            turn_speed: 3.,
            health: 1000,
        })
        .with(Team { team })
        .with(Radius { radius: 100. })
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::default(),
                    GlobalTransform::default(),
                    Barrel {},
                    Team { team },
                ))
                .with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(0.0, 15.0, 1.0)),
                        material: materials.add(barrel_beige.into()),
                        ..Default::default()
                    });
                });
        });
}
