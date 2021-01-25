use std::f32::consts::PI;

use bevy::prelude::*;

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
        .run();
}

struct Tank {
    speed: f32,
    turn_speed: f32,
}

struct Barrel {}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tank_beige = asset_server.load("sprites/Tanks/tankBeige.png");
    let barrel_beige = asset_server.load("sprites/Tanks/barrelBeige.png");

    let camera = Camera2dBundle::default();
    let e = commands.spawn(camera).current_entity().unwrap();

    commands.insert_resource(State {
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

fn tank_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Tank, &mut Transform)>,
) {
    for (tank, mut transform) in query.iter_mut() {
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

        transform.rotate(Quat::from_rotation_z(
            -turn_input * tank.turn_speed * time.delta_seconds(),
        ));
        let move_dir =
            transform.rotation * Vec3::unit_y() * drive * tank.speed * time.delta_seconds();
        transform.translation += move_dir;
    }
}

fn barrel_aim(
    mouse: Res<MousePosition>,
    mut query: Query<(&Barrel, &mut Transform, &GlobalTransform)>,
) {
    for (_barrel, mut transform, global_transform) in query.iter_mut() {

        let mut global_transform = global_transform.clone();
        global_transform.translation.z = 0.;
        let rel_pos = mouse.world - Vec2::new(global_transform.translation.x, global_transform.translation.y);
        let rel_angle = -rel_pos.x.atan2(rel_pos.y);

        let barrel_rot_pos = transform.rotation * Vec3::unit_y();
        let barrel_rot = -barrel_rot_pos.x.atan2(barrel_rot_pos.y);
        let world_rot_pos = global_transform.rotation * Vec3::unit_y();
        let world_rot = -world_rot_pos.x.atan2(world_rot_pos.y);

        println!("Rel: {} World: {} Barrel: {}", rel_angle, world_rot, barrel_rot);
        transform.rotation = Quat::from_rotation_z(rel_angle - world_rot + barrel_rot);
    }
}

struct State {
    cursor: EventReader<CursorMoved>,
    camera: Entity,
}

#[derive(Default)]
struct MousePosition {
    pixel: Vec2,
    world: Vec2,
}

fn mouse_position(
    mut state: ResMut<State>,
    mut mouse_pos: ResMut<MousePosition>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    // query to get camera components
    transforms: Query<&Transform>,
) {
    if let Some(cursor_event) = state.cursor.iter(&cursor_moved_events).last() {
        let camera_transform = transforms.get_component::<Transform>(state.camera).unwrap();

        // get the size of the window that the event is for
        let window = windows.get(cursor_event.id).unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let mouse_position = cursor_event.position - size / 2.0;

        // apply the camera transform
        let world = camera_transform.compute_matrix() * mouse_position.extend(0.0).extend(1.0);

        *mouse_pos = MousePosition {
            pixel: cursor_event.position,
            world: Vec2::new(world.x, world.y),
        };
    };
}
