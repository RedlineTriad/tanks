use bevy::{ecs::Entity, math::Vec2, prelude::EventReader, window::CursorMoved};

#[derive(Default)]
pub struct MousePosition {
    pub pixel: Vec2,
    pub world: Vec2,
}
pub struct MouseState {
    pub cursor: EventReader<CursorMoved>,
    pub camera: Entity,
}
