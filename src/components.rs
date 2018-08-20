use std::ops::{Deref, DerefMut};

use ggez::graphics;

use specs::{Entity, Component, NullStorage, VecStorage, WriteStorage};
use nphysics2d::math::{Point, Inertia, Isometry};
use nphysics2d::object::{BodyHandle, BodyStatus, RigidBody};

use resources::{PhysicWorld, BodiesMap};
use retained_storage::{RetainedStorage};

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

#[derive(Debug, Default)]
pub struct Controlable;

impl Component for Controlable {
    type Storage = NullStorage<Self>;
}

#[derive(Clone)]
pub struct CustomRigidBody(BodyHandle);
impl Component for CustomRigidBody {
    type Storage = RetainedStorage<Self, VecStorage<Self>>;
}
#[allow(unused)]
impl CustomRigidBody {
    pub fn safe_insert<'a>(
        entity: Entity,
        position: Isometry<f32>,
        local_inertia: Inertia<f32>,
        local_center_of_mass: Point<f32>,
        status: BodyStatus,
        bodies_handle: &mut WriteStorage<'a, CustomRigidBody>,
        physic_world: &mut PhysicWorld,
        bodies_map: &mut BodiesMap,
    ) -> Self {
        let body_handle =
            physic_world.add_rigid_body(position, local_inertia, local_center_of_mass);
        {
            let mut rigid_body = physic_world.rigid_body_mut(body_handle).unwrap();
            rigid_body.set_status(status);
            rigid_body
                .activation_status_mut()
                .set_deactivation_threshold(None);
        }
        bodies_map.insert(body_handle, entity);

        bodies_handle.insert(entity, CustomRigidBody(body_handle));
        CustomRigidBody(body_handle)
    }

    pub fn handle(&self) -> BodyHandle {
        self.0
    }

    #[inline]
    #[allow(unused)]
    pub fn get<'a>(
        &'a self,
        physic_world: &'a PhysicWorld,
    ) -> &'a RigidBody<f32> {
        physic_world
            .rigid_body(self.0)
            .expect("Rigid body in specs does not exist in physic world")
    }

    #[inline]
    pub fn get_mut<'a>(
        &self,
        physic_world: &'a mut PhysicWorld,
    ) -> &'a mut RigidBody<f32> {
        physic_world
            .rigid_body_mut(self.0)
            .expect("Rigid body in specs does not exist in physic world")
    }
}

#[derive(Debug)]
pub struct Contactor(pub Vec<Entity>);

impl Component for Contactor {
    type Storage = VecStorage<Self>;
}
impl Deref for Contactor {
    type Target = Vec<Entity>;
    fn deref(&self) -> &Vec<Entity> {
        &self.0
    }
}
impl DerefMut for Contactor {
    fn deref_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.0
    }
}
