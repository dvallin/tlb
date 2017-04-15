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
use engine::tcod::{ Tcod };

use tcod::colors::{ self };
use tcod::chars::{ self };

use tilemap::{ TileMap };

use components::appearance::{ Renderable };
use components::space::{ Position };
use components::control::{ PlayerControlled };

struct GameSystem;
unsafe impl Sync for GameSystem {}

impl System<()> for GameSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (players, mut positions, time, input, mut map) = arg.fetch(|w| {
            (w.read::<PlayerControlled>(),
             w.write::<Position>(),
             w.read_resource::<Time>(),
             w.read_resource::<InputHandler>(),
             w.write_resource::<TileMap>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        // proccess players
        for (player, position) in (&players, &mut positions).iter() {
            if input.is_pressed('h') {
                position.x -= delta_time;
            }
            if input.is_pressed('j') {
                position.y += delta_time;
            }
            if input.is_pressed('k') {
                position.y -= delta_time;
            }
            if input.is_pressed('l') {
                position.x += delta_time;
            }
        }
    }
}


const TORCH_RADIUS: i32 = 10;
struct Game;
impl State for Game {
    fn start(&mut self, tcod: &mut Tcod, world: &mut World) {
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

    fn update(&mut self, tcod: &mut Tcod, world: &mut World) -> Transition {
        let entities = world.entities();
        let players = world.read::<PlayerControlled>();
        let positions = world.read::<Position>();
        let mut tilemap = world.write_resource::<TileMap>();
        for (player, _entity, position) in (&players, &entities, &positions).iter() {
            tcod.compute_fov(position.x as i32, position.y as i32, TORCH_RADIUS);
        }

        tilemap.update(tcod);

        Transition::None
    }

    fn render(&mut self, tcod: &mut Tcod, world: &mut World) {
        let entities = world.entities();
        let renderables = world.read::<Renderable>();
        let positions = world.read::<Position>();
        let tilemap = world.read_resource::<TileMap>();
        let bgcolor = colors::BLACK;

        tcod.clear();

        {
            tilemap.draw(tcod);

            for (renderable, _entity, position) in (&renderables, &entities, &positions).iter() {
                tcod.render(position.x as i32, position.y as i32,
                            bgcolor, renderable.color, renderable.character);
            }
        }

        tcod.flush();
    }
}

fn main() {
    ApplicationBuilder::new(Game)
        .register::<PlayerControlled>()
        .with::<GameSystem>(GameSystem, "game_system", 1)
        .build()
        .run();

}
