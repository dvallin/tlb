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

fn main() {
    let mut planner : Planner<()> = Planner::new(World::new(), num_cpus::get());
    let mut state = StateMachine::new(Game);
    let mut tcod = Tcod::new();

    println!("Created World");
    let world = &mut planner.mut_world();
    world.register::<Renderable>();

    state.start(world);
    while state.is_running() {
        state.handle_events(world);
        state.update(world);
        tcod.render(world);
    }
}
