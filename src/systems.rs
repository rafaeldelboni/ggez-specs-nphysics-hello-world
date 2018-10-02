use ggez::graphics;
use ggez::graphics::{DrawParam, Drawable};
use ggez::{Context, GameResult};

use nphysics2d::algebra::{Velocity2};
use specs::{System, WriteStorage, ReadStorage, Read, Write, Join};

use resources::{InputControls, UpdateTime, PhysicWorld};
use components::{ShapeCuboid, ShapeTriangle, Velocity, Controlable, EcsRigidBody as Body};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Body>,
        Write<'a, PhysicWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (vel, mut body, mut phy_world) = data;
        (&vel, &mut body).join().for_each(|(vel, body)| {
            let b = body.get_mut(&mut phy_world);
            if vel.x != 0.0 && vel.y != 0.0 {
                let pi_inverse = 1.0 / (2.0 as f32).sqrt();
                b.set_velocity(Velocity2::linear(vel.x, vel.y) * pi_inverse);
            } else {
                b.set_velocity(Velocity2::linear(vel.x, vel.y));
            }
        });
    }
}

pub struct RenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
        RenderingSystem { ctx }
    }

    pub fn render(&mut self, points: &[graphics::Point2]) -> GameResult<()> {
        let mesh = graphics::Mesh::new_polygon(
            self.ctx,
            graphics::DrawMode::Line(0.1),
            points
        ).expect("Error creating polygon.");

        mesh.draw_ex(
            self.ctx,
            DrawParam {
                dest: graphics::Point2::origin(),
                rotation: 0.0,
                scale: graphics::Point2::new(10., 10.),
                offset: graphics::Point2::new(0.5, 0.5),
                ..Default::default()
            },
        )
    }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = (
        ReadStorage<'a, Body>,
        ReadStorage<'a, ShapeCuboid>,
        ReadStorage<'a, ShapeTriangle>,
        Read<'a, PhysicWorld>,
    );

    fn run(&mut self, (bodies, cube, triangle, world): Self::SystemData) {
        (&bodies, &cube).join().for_each(|(body, cube)| {
            let rbody = body.get(&world);

            let rect_x = rbody.position().translation.vector.x;
            let rect_y = rbody.position().translation.vector.y;
            let rect_w = cube.0.half_extents().x;
            let rect_h = cube.0.half_extents().y;

            let x1 = rect_x - rect_w;
            let x2 = rect_x + rect_w;
            let y1 = rect_y - rect_h;
            let y2 = rect_y + rect_h;
            let points = [
                graphics::Point2::new(x1, y1),
                graphics::Point2::new(x2, y1),
                graphics::Point2::new(x2, y2),
                graphics::Point2::new(x1, y2),
            ];

            self.render(&points).expect("Error drawing cube bounds.")
        });

        (&bodies, &triangle).join().for_each(|(body, triangle)| {
            let rbody = body.get(&world);

            let rect_x = rbody.position().translation.vector.x;
            let rect_y = rbody.position().translation.vector.y;
            let points = triangle.0.points();

            let x1 = rect_x + points[0].x;
            let x2 = rect_x + points[1].x;
            let x3 = rect_x + points[2].x;
            let y1 = rect_y + points[0].y;
            let y2 = rect_y + points[1].y;
            let y3 = rect_y + points[2].y;
            let points = [
                graphics::Point2::new(x1, y1),
                graphics::Point2::new(x2, y2),
                graphics::Point2::new(x3, y3),
            ];

            self.render(&points).expect("Error drawing triangle bounds.")
        });
    }
}

pub struct ControlableSystem;
impl<'a> System<'a> for ControlableSystem {
    type SystemData = (
        Read<'a, InputControls>,
        ReadStorage<'a, Controlable>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, (input, controlled, mut vel): Self::SystemData) {
        (&controlled, &mut vel).join().for_each(|(_ctrled, vel)| {
            if input.left {
                vel.x = -30.0;
            } else if input.right {
                vel.x = 30.0;
            } else {
                vel.x = 0.0;
            }
            if input.up {
                vel.y = -30.0;
            } else if input.down {
                vel.y = 30.0;
            } else {
                vel.y = 0.0;
            }
        });
    }
}

pub struct PhysicSystem;

impl<'a> System<'a> for PhysicSystem {
    type SystemData = (
        Read<'a, UpdateTime>,
        Write<'a, PhysicWorld>,
    );

    fn run(
        &mut self,
        (
            update_time,
            mut physic_world,
        ): Self::SystemData,
    ) {
        physic_world.set_timestep(update_time.0);
        physic_world.step();
    }
}
