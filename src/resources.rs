use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use specs::{Entity};
use nphysics2d::world::World;
use nphysics2d::object::{BodyHandle};

#[derive(Debug, Default)]
pub struct UpdateTime(pub f32);

pub type PhysicWorld = World<f32>;

#[derive(Debug, Default)]
pub struct BodiesMap(HashMap<BodyHandle, Entity>);

impl BodiesMap {
    pub fn new() -> Self {
        BodiesMap(HashMap::default())
    }
}
impl Deref for BodiesMap {
    type Target = HashMap<BodyHandle, Entity>;
    fn deref(&self) -> &HashMap<BodyHandle, Entity> {
        &self.0
    }
}
impl DerefMut for BodiesMap {
    fn deref_mut(&mut self) -> &mut HashMap<BodyHandle, Entity> {
        &mut self.0
    }
}
