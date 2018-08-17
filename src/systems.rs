use ggez::event;
use ggez::graphics;
use ggez::graphics::{Vector2};
use ggez::{Context};

use ncollide2d::events::{ContactEvent};
use nphysics2d::math::{Velocity as MathVelocity};

use specs::{System, WriteStorage, ReadStorage, Read, Write, Join};

use resources::{UpdateTime, PhysicWorld, BodiesMap};
use components::{Text, Velocity, Controlable, Contactor, CustomRigidBody as Body};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Body>,
        Write<'a, PhysicWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (vel, mut text, mut body, mut phy_world) = data;
        (&vel, &mut text, &mut body).join().for_each(|(vel, text, body)| {
            text.position.x += vel.x * 0.05;
            text.position.y += vel.y * 0.05;
            let v = Vector2::new(vel.x * 0.05, vel.y * 0.05);
            if let Some(v) = v.try_normalize(0.0001) {
                let body = body.get_mut(&mut phy_world);
                let current_angle = body.position().rotation.angle();
                let next_angle = -v[1].atan2(v[0]);
                body.apply_displacement(
                    &MathVelocity::angular(next_angle - current_angle));
                println!("body pos: {:#?}", body.position());
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
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = ReadStorage<'a, Text>;

    fn run(&mut self, texts: Self::SystemData) {
        &texts.join().for_each(|text| {
            graphics::draw(self.ctx, &text.value, text.position, 0.0).unwrap();
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
                        event::Keycode::Up => vel.y = -10.0,
                        event::Keycode::Down => vel.y = 10.0,
                        event::Keycode::Left => vel.x = -10.0,
                        event::Keycode::Right => vel.x = 10.0,
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
        WriteStorage<'a, Contactor>,
        Read<'a, UpdateTime>,
        Read<'a, BodiesMap>,
        Write<'a, PhysicWorld>,
    );

    fn run(
        &mut self,
        (
            mut contactors,
            update_time,
            bodies_map,
            mut physic_world,
        ): Self::SystemData,
    ) {
        physic_world.set_timestep(update_time.0);
        physic_world.step();
        for contact in physic_world.contact_events() {
            let collision_world = physic_world.collision_world();
            match contact {
                &ContactEvent::Started(coh1, coh2) => {
                    let bh1 = collision_world
                        .collision_object(coh1)
                        .unwrap()
                        .data()
                        .body();
                    let bh2 = collision_world
                        .collision_object(coh2)
                        .unwrap()
                        .data()
                        .body();
                    let e1 = *bodies_map.get(&bh1).unwrap();
                    let e2 = *bodies_map.get(&bh2).unwrap();
                    if let Some(contactor) = contactors.get_mut(e1) {
                        contactor.push(e2);
                    }
                    if let Some(contactor) = contactors.get_mut(e2) {
                        contactor.push(e1);
                    }
                }
                &ContactEvent::Stopped(coh1, coh2) => {
                    let bh1 = collision_world
                        .collision_object(coh1)
                        .unwrap()
                        .data()
                        .body();
                    let bh2 = collision_world
                        .collision_object(coh2)
                        .unwrap()
                        .data()
                        .body();
                    let e1 = *bodies_map.get(&bh1).unwrap();
                    let e2 = *bodies_map.get(&bh2).unwrap();
                    if let Some(contactor) = contactors.get_mut(e1) {
                        contactor.retain(|&e| e != e2);
                    }
                    if let Some(contactor) = contactors.get_mut(e2) {
                        contactor.retain(|&e| e != e1);
                    }
                }
            }
        }
    }
}
