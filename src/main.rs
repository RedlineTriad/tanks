use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Tanks".to_string(),
            vsync: false,
            ..Default::default()
        })
        .add_default_plugins()
        .init_resource::<MousePosition>()
        .add_system(mouse_position.system())
        .add_startup_system(setup.system())
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

    let camera = Camera2dComponents::default();
    let e = commands.spawn(camera).current_entity().unwrap();


    commands.insert_resource(State {
        cursor: Default::default(),
        camera: e
    });

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
        .with_children(|parent| {
            parent
                .spawn(SpriteComponents {
                    transform: Transform::from_translation(Vec3::new(0.0, 15.0, 1.0)),
                    material: materials.add(barrel_beige.into()),
                    ..Default::default()
                })
                .with(Barrel {});
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

        *transform.value_mut() = *transform.value()
            * Mat4::from_rotation_translation(
                Quat::from_rotation_z(-turn_input * tank.turn_speed * time.delta_seconds),
                Vec3::unit_y() * drive * tank.speed * time.delta_seconds,
            );
    }
}

fn barrel_aim(mouse: Res<MousePosition>, mut query: Query<(&Barrel, &mut Transform)>) {
    for (_barrel, mut transform) in &mut query.iter() {
        let translation = transform.translation();
        let angle = translation.angle_between(Vec3::new(
            mouse.world.x(),
            mouse.world.y(),
            0.,
        ));
        if angle.is_finite() {
            transform.set_rotation(Quat::from_rotation_z(-angle));
        }
    }
}

struct State {
    cursor: EventReader<CursorMoved>,
    camera: Entity
}

#[derive(Default)]
struct MousePosition{pixel: Vec2, world: Vec2}

fn mouse_position(
    mut state: ResMut<State>,
    mut mouse_pos: ResMut<MousePosition>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    // query to get camera components
    transforms: Query<&Transform>
) {
    if let Some(cursor_event) = state
        .cursor
        .iter(&cursor_moved_events)
        .last()
    {
        let camera_transform = transforms.get::<Transform>(state.camera).unwrap();

        // get the size of the window that the event is for
        let window = windows.get(cursor_event.id).unwrap();
        let size = Vec2::new(window.width as f32, window.height as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let mouse_position = cursor_event.position - size / 2.0;

        // apply the camera transform
        let world = *camera_transform.value() * mouse_position.extend(0.0).extend(1.0);
        println!("World: {:?}", world);

        *mouse_pos = MousePosition{
            pixel: cursor_event.position,
            world: Vec2::new(world.x(), world.y())
        };
    };
}
