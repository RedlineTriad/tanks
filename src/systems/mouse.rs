use bevy::{
    ecs::{Query, Res, ResMut},
    math::Vec2,
    prelude::{Events, Transform},
    window::{CursorMoved, Windows},
};

use crate::components::{MousePosition, MouseState};

pub fn mouse_position(
    mut state: ResMut<MouseState>,
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
