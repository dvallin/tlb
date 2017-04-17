extern crate tcod;
extern crate specs;
extern crate num_cpus;

mod engine;
mod components;
mod tilemap;
mod geometry;

use specs::{ World, Join };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ ApplicationBuilder };
use engine::tcod::{ Tcod };

use tcod::colors::{ self };

use tilemap::{ TileMap };

use components::appearance::{ Renderable };
use components::space::{ Position };
use components::control::{ PlayerController, Player, Fov };

const TORCH_RADIUS: i32 = 10;
struct Game;

impl Game {
    fn create_player(&mut self, index: usize, x: f32, y: f32, active: bool,
                     tcod: &mut Tcod, world: &mut World) {
        let fov_index = tcod.create_fov();
        tcod.initialize_fov(fov_index, world);
        world.create_now()
            .with(Position { x: x, y: y })
            .with(Renderable {character: '@', color: colors::WHITE })
            .with(Player {index: index, active: active})
            .with(Fov { index: fov_index })
            .build();
    }
}

impl State for Game {
    fn start(&mut self, tcod: &mut Tcod, world: &mut World) {
        world.add_resource::<InputHandler>(InputHandler::default());

        let mut map = TileMap::new();
        map.build();
        world.add_resource::<TileMap>(map);

        world.register::<Player>();
        world.register::<Fov>();

        self.create_player(1, 15.0, 15.0, true, tcod, world);
        self.create_player(2, 16.0, 16.0, false, tcod, world);
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
        let fovs = world.read::<Fov>();
        let positions = world.read::<Position>();
        let mut tilemap = world.write_resource::<TileMap>();
        for (fov, _entity, position) in (&fovs, &entities, &positions).iter() {
            tcod.compute_fov(fov.index, position.x as i32, position.y as i32, TORCH_RADIUS);
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
        .with::<PlayerController>(PlayerController, "player_controller_system", 1)
        .build()
        .run();

}
