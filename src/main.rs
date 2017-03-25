extern crate tcod;
extern crate specs;
extern crate num_cpus;

mod engine;
mod components;
mod tilemap;

use specs::{ World, Planner };

use engine::state::{ StateMachine, State, Transition };
use engine::input_handler::{ InputHandler };
use engine::tcod::{ Tcod };
use components::renderable::{ Renderable };
use components::position::{ Position };
use tilemap::{ TileMap };

struct Game;
impl State for Game {
    fn start(&mut self, world: &mut World) {
        println!("Starte Game");
        world.add_resource::<InputHandler>(InputHandler::default());
        world.add_resource::<TileMap>(TileMap::new())
    }

    fn handle_events(&mut self, world: &mut World) -> Transition {
        let mut input = world.write_resource::<InputHandler>();
        input.update();
        match input.key {
            tcod::input::Key { code: tcod::input::KeyCode::Escape, ..} => return Transition::Exit,
            _ => (),
        }
        Transition::None
    }
}

use std::time::{ Duration, Instant };
enum Stopwatch {
    Waiting,
    Started(Duration, Instant),
    Ended(Duration),
}

impl Default for Stopwatch {
    fn default() -> Self {
        Stopwatch::Waiting
    }
}

impl Stopwatch {
    pub fn elapsed(&self) -> Duration {
        match *self {
            Stopwatch::Waiting => Duration::new(0, 0),
            Stopwatch::Started(duration, start) => duration + start.elapsed(),
            Stopwatch::Ended(duration) => duration,
        }
    }

    pub fn start(&mut self) {
        match self {
            &mut Stopwatch::Waiting => self.restart(),
            &mut Stopwatch::Ended(duration) => {
                *self = Stopwatch::Started(duration, Instant::now())
            },
            _ => {}
        }
    }

    pub fn restart(&mut self) {
        *self = Stopwatch::Started(Duration::new(0, 0), Instant::now())
    }

    pub fn stop(&mut self) {
        if let &mut Stopwatch::Started(duration, start) = self {
            *self = Stopwatch::Ended(duration + start.elapsed());
        }
    }
}

struct Application {
    planner: Planner<()>,
    state: StateMachine,
    tcod: Tcod,

    timer: Stopwatch,
    delta_time: Duration,
}

impl Application {
    pub fn new() -> Self {
        Application {
            planner: Planner::new(World::new(), num_cpus::get()),
            state: StateMachine::new(Game),
            tcod: Tcod::new(),
            timer: Stopwatch::default(),
            delta_time: Duration::new(0, 0),
        }
    }

    pub fn run(&mut self) {
        self.initialize();
        println!("Created World");
        while self.state.is_running() {
            self.timer.restart();
            self.step();
            self.timer.stop();
            self.delta_time = self.timer.elapsed();
        }
    }

    fn step(&mut self) {
        {
            let world = self.planner.mut_world();
            world.register::<Renderable>();
            world.register::<Position>();
            self.state.handle_events(world);
            self.state.update(world);
        }

        self.planner.dispatch(());
        self.planner.wait();

        {
            let world = self.planner.mut_world();
            self.tcod.render(world);
        }
    }

    fn initialize(&mut self) {
        let world = self.planner.mut_world();
        self.state.start(world);
    }
}

fn main() {
    Application::new()
        .run();

}
