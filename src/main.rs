extern crate ggez;
extern crate specs;
extern crate hibitset;
extern crate nphysics2d;
extern crate ncollide2d;
extern crate nalgebra;

mod entities;
mod systems;
mod components;
mod resources;
mod retained_storage;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use nphysics2d::math::Vector;
use specs::{Dispatcher, DispatcherBuilder, World, RunNow};

use retained_storage::Retained;
use systems::{ControlSystem, RenderingSystem, MoveSystem};
use components::{Controlable, Text, Velocity, CustomRigidBody, Contactor};
use resources::{BodiesMap, PhysicWorld, UpdateTime};

struct MainState<'a, 'b> {
    frames: usize,
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

        let mut world = World::new();
        world.register::<Text>();
        world.register::<Velocity>();
        world.register::<Controlable>();
        world.register::<CustomRigidBody>();
        world.register::<Contactor>();

        let mut physic_world = PhysicWorld::new();
        physic_world.set_gravity(Vector::new(0.0, 0.0));
        world.add_resource(physic_world);
        world.add_resource(BodiesMap::new());
        world.add_resource(UpdateTime(0.0));

        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .with(MoveSystem, "move_system", &[])
            .build();

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;

        entities::create_static(ctx, &mut world, &font);
        entities::create_moving(ctx, &mut world, &font);
        entities::create_controled(ctx, &mut world, &font);

        Ok(MainState {
            frames: 0,
            world,
            dispatcher
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let last_update_instant = std::time::Instant::now();
        let delta_time = last_update_instant.elapsed();

        self.world.write_resource::<UpdateTime>().0 = delta_time
            .as_secs()
            .saturating_mul(1_000_000_000)
            .saturating_add(delta_time.subsec_nanos() as u64)
            as f32 / 1_000_000_000.0;

        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        let mut physic_world = self.world.write_resource::<PhysicWorld>();
        let mut bodies_map = self.world.write_resource::<BodiesMap>();

        let retained = self.world
            .write_storage::<CustomRigidBody>()
            .retained()
            .iter()
            .map(|r| r.handle())
            .collect::<Vec<_>>();

        physic_world.remove_bodies(&retained);

        for handle in &retained {
            bodies_map.remove(handle);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let mut rs = RenderingSystem::new(ctx);
            rs.run_now(&mut self.world.res);
        }

        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool
    ) {
        let mut cs = ControlSystem::new(keycode, true);
        cs.run_now(&mut self.world.res);
    }

    fn key_up_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool

    ) {
        let mut cs = ControlSystem::new(keycode, false);
        cs.run_now(&mut self.world.res);
    }
}

fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);

    let ctx = &mut Context::load_from_conf("ggez-specs-hello-world", "ggez", c).unwrap();

    match MainState::new(ctx) {
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
    }
}
