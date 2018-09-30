use ggez::{Context};
use specs::{Builder, World};
use ncollide2d::shape::{ShapeHandle, Cuboid};
use nphysics2d::object::{BodyStatus, Material};
use nphysics2d::volumetric::Volumetric;
use nalgebra::{Isometry2, Vector2};

use resources::{PhysicWorld};
use components::{Controlable, Collider, Velocity, CustomRigidBody};

pub fn create_static(_ctx: &mut Context, world: &mut World, x: f32, y: f32) {
    let collider = Collider::new(Vector2::new(5.0, 5.0));

    let entity = world.create_entity()
        .with(collider.clone())
        .with(Velocity { x: 0., y: 0. })
        .build();

    let mut physic_world = world.write_resource::<PhysicWorld>();

    let shape = ShapeHandle::new(Cuboid::new(collider.half_size));
    let mut inertia = shape.inertia(1.0);
    inertia.angular = 0.0;
    let center_of_mass = shape.center_of_mass();

    let body_handle = CustomRigidBody::safe_insert(
        entity,
        Isometry2::new(Vector2::new(x, y), 0.0),
        inertia,
        center_of_mass,
        BodyStatus::Static,
        &mut world.write_storage(),
        &mut physic_world,
        &mut world.write_resource(),
    );

    physic_world.add_collider(
        0.01,
        shape,
        body_handle.handle(),
        Isometry2::identity(),
        Material::default(),
    ); 
 }

pub fn create_moving(_ctx: &mut Context, world: &mut World) {
    let collider = Collider::new(Vector2::new(5.0, 2.5));

    let entity = world
        .create_entity()
        .with(collider.clone())
        .with(Velocity { x: 10., y: 10. })
        .build();

    let mut physic_world = world.write_resource::<PhysicWorld>();

    let shape = ShapeHandle::new(Cuboid::new(collider.half_size));
    let mut inertia = shape.inertia(1.0);
    inertia.angular = 0.0;
    let center_of_mass = shape.center_of_mass();

    let body_handle = CustomRigidBody::safe_insert(
        entity,
        Isometry2::new(Vector2::new(1.0, 8.0), 0.0),
        inertia,
        center_of_mass,
        BodyStatus::Dynamic,
        &mut world.write_storage(),
        &mut physic_world,
        &mut world.write_resource(),
    );

    physic_world.add_collider(
        0.01,
        shape,
        body_handle.handle(),
        Isometry2::identity(),
        Material::default(),
    ); 
}

pub fn create_controled(_ctx: &mut Context, world: &mut World) {
    let collider = Collider::new(Vector2::new(2.0, 2.0));

    let entity = world
        .create_entity()
        .with(collider.clone())
        .with(Velocity { x: 0., y: 0. })
        .with(Controlable)
        .build();

    let mut physic_world = world.write_resource::<PhysicWorld>();

    let shape = ShapeHandle::new(Cuboid::new(collider.half_size));
    let mut inertia = shape.inertia(1.0);
    inertia.angular = 0.0;
    let center_of_mass = shape.center_of_mass();

    let body_handle = CustomRigidBody::safe_insert(
        entity,
        Isometry2::new(Vector2::new(2.0, 40.0), 0.0),
        inertia,
        center_of_mass,
        BodyStatus::Dynamic,
        &mut world.write_storage(),
        &mut physic_world,
        &mut world.write_resource(),
    );

    physic_world.add_collider(
        0.01,
        shape,
        body_handle.handle(),
        Isometry2::identity(),
        Material::default(),
    ); 
}
