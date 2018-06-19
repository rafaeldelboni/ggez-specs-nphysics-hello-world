use ggez::graphics;

use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Text {
    pub value: graphics::Text,
    pub position: graphics::Point2,
}

impl Component for Text {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Controlable;

impl Component for Controlable {
    type Storage = VecStorage<Self>;
}

