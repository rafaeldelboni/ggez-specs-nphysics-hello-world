extern crate ggez;
extern crate specs;

mod systems;
mod components;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use specs::{World, RunNow};

use systems::{Systems, ControlSystem, RenderingSystem, MoveSystem};
use components::{Controlable, Text, Velocity};

struct MainState {
    frames: usize,
    world: World,
    systems: Systems,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

        let mut world = World::new();
        world.register::<Text>();
        world.register::<Velocity>();
        world.register::<Controlable>();

        let systems = Systems {
            move_system: MoveSystem,
        };

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;

        world
            .create_entity()
            .with(Text {
                value: graphics::Text::new(ctx, "Static text!", &font)?,
                position: graphics::Point2::new(10.0, 10.0)})
            .build();

        world
            .create_entity()
            .with(Text {
                value: graphics::Text::new(
                           ctx,
                           "I'm a moving alone text!",
                           &font)?,
                position: graphics::Point2::new(20.0, 200.0)})
            .with(Velocity { x: 5., y: 5. })
            .build();

        world
            .create_entity()
            .with(Text {
                value: graphics::Text::new(ctx, "Move-me text!", &font)?,
                position: graphics::Point2::new(20.0, 400.0)})
            .with(Velocity { x: 0., y: 0. })
            .with(Controlable)
            .build();

        let state = MainState {
            frames: 0,
            world,
            systems,
        };

        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.systems.move_system.run_now(&self.world.res);
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
