extern crate tcod;
extern crate specs;
extern crate num_cpus;

mod engine;
mod components;
mod tilemap;
mod geometry;

use specs::{ World, System, RunArg, Join };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ ApplicationBuilder };
use engine::time::{ Time };

use tcod::colors::{ self };

use tilemap::{ TileMap };

use components::appearance::{ Renderable };
use components::space::{ Position };
use components::control::{ PlayerControlled };

struct GameSystem;
unsafe impl Sync for GameSystem {}

impl System<()> for GameSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (players, mut positions, time, input) = arg.fetch(|w| {
            (w.read::<PlayerControlled>(),
             w.write::<Position>(),
             w.read_resource::<Time>(),
             w.read_resource::<InputHandler>())
        });

        // proccess players
        for (player, position) in (&players, &mut positions).iter() {
            match input.key {
                tcod::input::Key { printable: 'h', ..} => position.x -= 1.0,
                tcod::input::Key { printable: 'j', ..} => position.y -= 1.0,
                tcod::input::Key { printable: 'k', ..} => position.y += 1.0,
                tcod::input::Key { printable: 'l', ..} => position.x += 1.0,
                _ => (),
            }
        }
    }
}

struct Game;
impl State for Game {
    fn start(&mut self, world: &mut World) {
        world.add_resource::<InputHandler>(InputHandler::default());

        let mut map = TileMap::new();
        map.build();
        world.add_resource::<TileMap>(map);

        world.register::<PlayerControlled>();

        world.create_now()
            .with(Position { x: 15.0, y: 15.0 })
            .with(Renderable {character: '@', color: colors::WHITE })
            .with(PlayerControlled {})
            .build();
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
    ApplicationBuilder::new(Game)
        .register::<PlayerControlled>()
        .with::<GameSystem>(GameSystem, "game_system", 1)
        .build()
        .run();

}
