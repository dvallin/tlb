extern crate tcod;
extern crate specs;
extern crate num_cpus;

mod engine;
mod components;
mod tilemap;

use specs::{ World };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ Application };

use tilemap::{ TileMap };

struct Game;
impl State for Game {
    fn start(&mut self, world: &mut World) {
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
    Application::new(Game)
        .run();

}
