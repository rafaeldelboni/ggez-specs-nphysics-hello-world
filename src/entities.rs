use ggez::graphics;
use ggez::{Context};
use ggez::graphics::{Font};
use specs::{Builder, World};
use ncollide2d::shape::{ShapeHandle, Cuboid};
use nphysics2d::object::{BodyStatus, Material};
use nphysics2d::math::{Point, Inertia, Isometry};
use nalgebra::{one, Isometry2, Vector2};

use resources::{PhysicWorld};
use components::{Controlable, Text, Velocity, CustomRigidBody};

pub fn create_static(ctx: &mut Context, world: &mut World, font: &Font) {
    let entity = world.create_entity()
        .with(Text {
            value: graphics::Text::new(ctx, "Static text!", &font).unwrap(),
            position: graphics::Point2::new(10.0, 10.0)})
        .build();

    let mut physic_world = world.write_resource::<PhysicWorld>();

    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(2.0, 1.0)));

    let body_handle = CustomRigidBody::safe_insert(
        entity,
        Isometry2::new(Vector2::new(0.0, 0.0), 0.0),
        Inertia::new(2.0, 2.0),
        Point::new(0.5, 0.5),
        BodyStatus::Dynamic,
        &mut world.write_storage(),
        &mut physic_world,
        &mut world.write_resource(),
    );

    physic_world.add_collider(
        0.0,
        shape,
        body_handle.handle(),
        one(),
        Material::new(0.5, 1.0),
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
            position: graphics::Point2::new(20.0, 200.0)
        })
        .with(Velocity { x: 5., y: 5. })
        .build();
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
}
