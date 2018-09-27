use ggez::graphics;
use ggez::{Context};
use ggez::graphics::{Font};
use specs::{Builder, World};
use ncollide2d::shape::{ShapeHandle, Cuboid};
use nphysics2d::object::{BodyStatus, Material};
use nphysics2d::volumetric::Volumetric;
use nalgebra::{Isometry2, Vector2};

use resources::{PhysicWorld};
use components::{Controlable, Text, Velocity, CustomRigidBody};

pub fn create_static(ctx: &mut Context, world: &mut World, font: &Font, x: f32, y: f32) {
    let entity = world.create_entity()
        .with(Text {
            value: graphics::Text::new(ctx, "Static text!", &font).unwrap(),
            position: graphics::Point2::new(x, y)})
        .with(Velocity { x: 0., y: 0. })
        .build();

    let mut physic_world = world.write_resource::<PhysicWorld>();

    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
        24.9,
        24.9,
    )));
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
        0.1,
        shape,
        body_handle.handle(),
        Isometry2::identity(),
        Material::default(),
    ); 
 }

pub fn create_moving(ctx: &mut Context, world: &mut World, font: &Font) {
    let entity = world
        .create_entity()
        .with(Text {
            value: graphics::Text::new(
                ctx,
                   "I'm a moving alone text!",
                   &font
               ).unwrap(),
            position: graphics::Point2::new(10.0, 80.0)
        })
        .with(Velocity { x: 50., y: 50. })
        .build();

    let mut physic_world = world.write_resource::<PhysicWorld>();

    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
        24.9,
        24.9,
    )));
    let mut inertia = shape.inertia(1.0);
    inertia.angular = 0.0;
    let center_of_mass = shape.center_of_mass();

    let body_handle = CustomRigidBody::safe_insert(
        entity,
        Isometry2::new(Vector2::new(10.0, 80.0), 0.0),
        inertia,
        center_of_mass,
        BodyStatus::Dynamic,
        &mut world.write_storage(),
        &mut physic_world,
        &mut world.write_resource(),
    );

    physic_world.add_collider(
        0.1,
        shape,
        body_handle.handle(),
        Isometry2::identity(),
        Material::default(),
    ); 
}

pub fn create_controled(ctx: &mut Context, world: &mut World, font: &Font) {
    let entity = world
        .create_entity()
        .with(Text {
            value: graphics::Text::new(ctx, "Move-me text!", &font).unwrap(),
            position: graphics::Point2::new(20.0, 400.0)})
        .with(Velocity { x: 0., y: 0. })
        .with(Controlable)
        .build();

    let mut physic_world = world.write_resource::<PhysicWorld>();

    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
        24.9,
        24.9,
    )));
    let mut inertia = shape.inertia(1.0);
    inertia.angular = 0.0;
    let center_of_mass = shape.center_of_mass();

    let body_handle = CustomRigidBody::safe_insert(
        entity,
        Isometry2::new(Vector2::new(20.0, 400.0), 0.0),
        inertia,
        center_of_mass,
        BodyStatus::Dynamic,
        &mut world.write_storage(),
        &mut physic_world,
        &mut world.write_resource(),
    );

    physic_world.add_collider(
        0.1,
        shape,
        body_handle.handle(),
        Isometry2::identity(),
        Material::default(),
    ); 
}
