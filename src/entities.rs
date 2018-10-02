use ggez::{Context};
use specs::{Builder, World};
use ncollide2d::shape::{Cuboid, ConvexPolygon};
use nphysics2d::object::{BodyStatus};
use nalgebra::{Point2, Vector2};

use components::{ShapeBase, ShapeCuboid, ShapeTriangle, Controlable, Velocity};

pub fn create_static_tri(_ctx: &mut Context, world: &mut World, x: f32, y: f32) {
    let triangle = ShapeTriangle(
        ConvexPolygon::try_from_points(&[
            Point2::new(-2.0, -2.0),
            Point2::new(2.0, 2.0),
            Point2::new(-2.0, 2.0),
        ]).expect("Convex hull computation failed.")
    );

    let entity = world.create_entity()
        .with(triangle.clone())
        .with(Velocity { x: 0., y: 0. })
        .build();

    triangle.create_rigid_body(
        world,
        entity,
        BodyStatus::Static,
        Vector2::new(x, y)
    );
}

pub fn create_static(_ctx: &mut Context, world: &mut World, x: f32, y: f32) {
    let cuboid = ShapeCuboid(Cuboid::new(Vector2::new(2.5, 2.5)));

    let entity = world.create_entity()
        .with(cuboid.clone())
        .with(Velocity { x: 0., y: 0. })
        .build();

    cuboid.create_rigid_body(
        world,
        entity,
        BodyStatus::Static,
        Vector2::new(x, y)
    );
}

pub fn create_moving(_ctx: &mut Context, world: &mut World) {
    let cuboid = ShapeCuboid(Cuboid::new(Vector2::new(2.5, 1.25)));

    let entity = world
        .create_entity()
        .with(cuboid.clone())
        .with(Velocity { x: 10., y: 10. })
        .build();

    cuboid.create_rigid_body(
        world,
        entity,
        BodyStatus::Dynamic,
        Vector2::new(1.0, 8.0)
    );
}

pub fn create_controled(_ctx: &mut Context, world: &mut World) {
    let cuboid = ShapeCuboid(Cuboid::new(Vector2::new(1.0, 1.0)));

    let entity = world
        .create_entity()
        .with(cuboid.clone())
        .with(Velocity { x: 0., y: 0. })
        .with(Controlable)
        .build();

    cuboid.create_rigid_body(
        world,
        entity,
        BodyStatus::Dynamic,
        Vector2::new(2.0, 40.0)
    );
}
