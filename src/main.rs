use bevy::prelude::*;
use components::{Barrel, Health, MousePosition, MouseState, Radius, Tank, Team};
use systems::{
    barrel_aim, death, lifetime, lifetime_destroy, mouse_position, move_forward, shoot, tank_hit,
    tank_movement,
};
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
        .add_system(death.system())
        .add_system(lifetime.system())
        .add_system(lifetime_destroy.system())
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

    for x in -2..2 {
        for y in -2..2 {
            create_tank(
                commands,
                &mut materials,
                &mut asset_server,
                Transform {
                    translation: Vec3::new(x as f32 * 80., y as f32 * 80., 0.),
                    ..Default::default()
                },
                Team { team: x + y },
            );
        }
    }
}

fn create_tank(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &mut Res<AssetServer>,
    transform: Transform,
    team: Team,
) {
    let tank_beige =
        asset_server.load(&format!("sprites/Tanks/tank{}.png", team.color().name())[..]);
    let barrel_beige =
        asset_server.load(&format!("sprites/Tanks/barrel{}.png", team.color().name())[..]);
    commands
        .spawn(SpriteBundle {
            material: materials.add(tank_beige.into()),
            transform,
            ..Default::default()
        })
        .with(Tank {
            speed: 500.,
            turn_speed: 3.,
        })
        .with(Health(1000))
        .with(team)
        .with(Radius { radius: 30. })
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::default(),
                    GlobalTransform::default(),
                    Barrel {},
                    team,
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
