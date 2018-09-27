use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawParam, Drawable};
use ggez::{Context};

use nalgebra::{Vector2};
use specs::{System, WriteStorage, ReadStorage, Read, Write, Join};

use resources::{UpdateTime, PhysicWorld};
use components::{Text, Velocity, Controlable, CustomRigidBody as Body};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Read<'a, UpdateTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Body>,
        Write<'a, PhysicWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut text, mut body, mut phy_world) = data;
        (&vel, &mut text, &mut body).join().for_each(|(vel, text, body)| {
            let v = Vector2::new(vel.x, vel.y) * delta.0;
            let b = body.get_mut(&mut phy_world);
            b.set_linear_velocity(v);
            text.position.x = b.position().translation.vector.x;
            text.position.y = b.position().translation.vector.y;
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
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = (
        ReadStorage<'a, Body>,
        Read<'a, PhysicWorld>,
    );

    fn run(&mut self, (bodies, world): Self::SystemData) {
        (&bodies).join().for_each(|body| {
            let rbody = body.get(&world);

            let rect_x = rbody.position().translation.vector.x;
            let rect_y = rbody.position().translation.vector.y;
            let rect_w = 5.0;
            let rect_h = 5.0;

            let x1 = rect_x;
            let x2 = rect_x + rect_w;
            let y1 = rect_y;
            let y2 = rect_y + rect_h;
            let points = [
                graphics::Point2::new(x1*10.0, y1*10.0),
                graphics::Point2::new(x2*10.0, y1*10.0),
                graphics::Point2::new(x2*10.0, y2*10.0),
                graphics::Point2::new(x1*10.0, y2*10.0),
            ];
            let mesh = graphics::Mesh::new_polygon(
                self.ctx,
                graphics::DrawMode::Line(1.0),
                &points
            ).expect("Error creating polygon.");

            mesh.draw_ex(
                self.ctx,
                DrawParam {
                    dest: graphics::Point2::origin(),
                    rotation: 0.0,
                    ..Default::default()
                },
            ).expect("Error drawing entity bounds.");
        });
    }
}

pub struct ControlSystem {
    keycode: event::Keycode,
    down_event: bool,
}

impl ControlSystem {
    pub fn new(keycode: event::Keycode, down_event: bool) -> ControlSystem {
        ControlSystem { keycode, down_event }
    }
}

impl<'a> System<'a> for ControlSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Controlable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut velocities, controlables) = data;
        for (vel, _control) in (&mut velocities, &controlables).join() {
            match self.down_event {
                true =>
                    match self.keycode {
                        event::Keycode::Up => vel.y = -500.0,
                        event::Keycode::Down => vel.y = 500.0,
                        event::Keycode::Left => vel.x = -500.0,
                        event::Keycode::Right => vel.x = 500.0,
                        _ => {}
                    }
                false =>
                    match self.keycode {
                        event::Keycode::Up => vel.y = 0.0,
                        event::Keycode::Down => vel.y = 0.0,
                        event::Keycode::Left => vel.x = 0.0,
                        event::Keycode::Right => vel.x = 0.0,
                        _ => {}
                    }
            }
        }
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
        println!("{}", update_time.0);
        physic_world.set_timestep(0.1 - update_time.0);
        physic_world.step();
    }
}
