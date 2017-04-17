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

use tilemap::{ TileMap };

use components::appearance::{ Renderable };
use components::space::{ Position, Vector, mul };
use components::control::{ Player, Fov };

const PLAYER_SPEED: f32 = 4.0;

struct GameSystem;
unsafe impl Sync for GameSystem {}

impl System<()> for GameSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (players, mut positions, time, input) = arg.fetch(|w| {
            (w.read::<Player>(),
             w.write::<Position>(),
             w.read_resource::<Time>(),
             w.read_resource::<InputHandler>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        // proccess players
        for (player, position) in (&players, &mut positions).iter() {
            if player.active {
                let mut delta = Vector { x: 0.0, y: 0.0 };
                if input.is_pressed('h') {
                    delta.x -= 1.0;
                }
                if input.is_pressed('j') {
                    delta.y += 1.0;
                }
                if input.is_pressed('k') {
                    delta.y -= 1.0;
                }
                if input.is_pressed('l') {
                    delta.x += 1.0;
                }
                *position += mul(delta.norm(), delta_time*PLAYER_SPEED)
            }
        }
    }
}


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

    fn activate_player(&mut self, index: usize, world: &mut World) {
        let mut players = world.write::<Player>();
        for player in (&mut players).iter() {
            player.active = player.index == index;
        }
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
        let mut switch_player : Option<usize> = None;
        {
            let mut input = world.write_resource::<InputHandler>();
            input.update();
            match input.key {
                tcod::input::Key { code: tcod::input::KeyCode::Escape, ..} => return Transition::Exit,
                _ => (),
            }
            if input.is_pressed('1') {
                switch_player = Some(1);
            } else if input.is_pressed('2') {
                switch_player = Some(2);
            }
        }

        if let Some(index) = switch_player {
            self.activate_player(index, world);
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
        .with::<GameSystem>(GameSystem, "game_system", 1)
        .build()
        .run();

}
