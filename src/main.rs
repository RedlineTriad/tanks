use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(tank_movement.system())
        .run();
}

struct Tank {
    speed: f32,
    turn_speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tank_beige = asset_server
        .load("assets/sprites/Tanks/tankBeige.png")
        .unwrap();
    let barrel_beige = asset_server
        .load("assets/sprites/Tanks/barrelBeige.png")
        .unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(tank_beige.into()),
            ..Default::default()
        })
        .with(Tank {
            speed: 500.,
            turn_speed: 3.,
        })
        .with_children(|c| {
            c.spawn(SpriteComponents {
                material: materials.add(barrel_beige.into()),
                ..Default::default()
            });
        });
}

fn tank_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Tank, &mut Transform)>,
) {
    for (tank, mut transform) in &mut query.iter() {
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

        *transform.value_mut() = *transform.value() * Mat4::from_rotation_translation(
            Quat::from_rotation_z(-turn_input * tank.turn_speed * time.delta_seconds), 
            Vec3::unit_y() * drive * tank.speed * time.delta_seconds);
    }
}
