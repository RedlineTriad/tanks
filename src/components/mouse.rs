use bevy::{math::Vec2, prelude::Entity};

#[derive(Default)]
pub struct MousePosition {
    pub pixel: Vec2,
    pub world: Vec2,
}

pub struct MainCamera {
    pub camera: Entity,
}
