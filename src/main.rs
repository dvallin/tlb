extern crate tcod;
extern crate specs;
extern crate num_cpus;

mod engine;
mod components;
mod tilemap;
mod ui;
mod geometry;
mod systems;
mod itemmap;
mod game_stats;

use specs::{ World, Join };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ ApplicationBuilder };
use engine::tcod::{ Tcod };

use tcod::colors::{ self };
use tcod::input::{ KeyCode };

use tilemap::{ TileMap };
use game_stats::{ GameStats };
use ui::{ Ui };

use components::appearance::{ Renderable, Layer0, Layer1 };
use components::space::{ Position, Viewport };
use components::player::{ Player, Fov };
use components::common::{ Health, Description };
use components::inventory::{ Inventory };

use geometry::{ Pos, Rect };

use systems::player_controller::{ PlayerController };
use systems::ui::{ UiUpdater };

use itemmap::{ Item, ItemInstance, ItemMap };

const TORCH_RADIUS: i32 = 10;
struct Game;

impl Game {
    fn create_player(&mut self, x: f32, y: f32, active: bool, name: String,
                     tcod: &mut Tcod, world: &mut World) {
        let fov_index = tcod.create_fov();
        tcod.initialize_fov(fov_index, world);

        let pos = Position { x: x, y: y };
        let p = Player { active: active, spawn: pos };
        world.create_now()
            .with(Renderable { character: '@', color: colors::WHITE })
            .with(Health { health: 100.0 } )
            .with(Description { name: name, description: "".into() })
            .with(Fov { index: fov_index })
            .with(Inventory::new())
            .with(Layer1)
            .with(p)
            .with(pos)
            .build();
    }

    fn create_item(&mut self, x: f32, y: f32, instance: ItemInstance, world: &mut World) {
        let pos = Position { x: x, y: y };
        let i = Item { instance: instance, spawn: pos };
        world.create_now()
            .with(itemmap::get_renderable(&i))
            .with(itemmap::get_description(&i))
            .with(Layer0)
            .with(i)
            .with(pos)
            .build();
    }

    fn reset_world(&mut self, world: &mut World) {
        let entities = world.entities();
        let mut stats = world.write_resource::<GameStats>();
        let mut positions = world.write::<Position>();
        let mut item_map = world.write_resource::<ItemMap>();

        let players = world.read::<Player>();
        let items = world.read::<Item>();

        for (player, position) in (&players, &mut positions).iter() {
            *position = Position { x: player.spawn.x, y: player.spawn.y };
        }

        item_map.clear();
        for (id, item, position) in (&entities, &items, &mut positions).iter() {
            *position = Position { x: item.spawn.x, y: item.spawn.y };
            item_map.push(&id, item.spawn.x as i32, item.spawn.y as i32);
        }

        stats.reset();
    }
}

fn render_into_viewport(viewport: &Viewport, bgcolor: tcod::Color, position: &Position,
                        renderable: &Renderable, tcod: &mut Tcod) {
    let p = Pos { x: position.x as i32, y: position.y as i32 };
    if viewport.visible(p) {
        let pos = viewport.transform(p);
        tcod.render(pos.x, pos.y, bgcolor, renderable.color, renderable.character);
    }
}

impl State for Game {
    fn start(&mut self, tcod: &mut Tcod, world: &mut World) {
        world.add_resource::<InputHandler>(InputHandler::default());
        world.add_resource::<GameStats>(GameStats::default());

        let mut tile_map = TileMap::new();
        tile_map.build();
        world.add_resource::<TileMap>(tile_map);
        world.add_resource::<ItemMap>(ItemMap::new());

        self.create_item(14.0, 15.0, ItemInstance::FlickKnife, world);

        self.create_player(15.0, 15.0, true, "Colton".into(), tcod, world);
        self.create_player(16.0, 16.0, false, "Gage".into(), tcod, world);

        self.reset_world(world);

        let mut ui = Ui::new();
        ui.add("active_player".into(), Rect::new(1, 1, 11, 2));
        ui.add("time_left".into(), Rect::new(34, 1, 11, 1));
        ui.add("inactive_player".into(), Rect::new(67, 1, 11, 2));
        world.add_resource::<Ui>(ui);

        let viewport = Viewport::new(15, 15, 80, 40);
        world.add_resource::<Viewport>(viewport);
    }

    fn handle_events(&mut self, world: &mut World) -> Transition {
        let mut input = world.write_resource::<InputHandler>();
        input.update();
        if input.is_key_pressed(KeyCode::Escape) {
            return Transition::Exit
        }
        Transition::None
    }

    fn update(&mut self, tcod: &mut Tcod, world: &mut World) -> Transition {
        let do_reset;
        {
            let input = world.read_resource::<InputHandler>();
            let stats = world.read_resource::<GameStats>();
            do_reset = input.is_key_pressed(KeyCode::Backspace) || stats.time_left() < 0;
        }
        if do_reset {
            self.reset_world(world);
        }

        let fovs = world.read::<Fov>();
        let positions = world.read::<Position>();
        let mut tilemap = world.write_resource::<TileMap>();
        for (fov, position) in (&fovs, &positions).iter() {
            tcod.compute_fov(fov.index, position.x as i32, position.y as i32, TORCH_RADIUS);
        }

        tilemap.update(tcod);

        Transition::None
    }

    fn render(&mut self, tcod: &mut Tcod, world: &mut World) {
        let renderables = world.read::<Renderable>();
        let positions = world.read::<Position>();
        let layer0 = world.read::<Layer0>();
        let layer1 = world.read::<Layer1>();
        let tilemap = world.read_resource::<TileMap>();
        let ui = world.read_resource::<Ui>();
        let viewport = world.read_resource::<Viewport>();
        let bgcolor = colors::BLACK;

        tcod.clear(bgcolor);

        {
            tilemap.draw(tcod, &viewport);
            ui.draw(tcod);

            for (_, renderable, position) in (&layer0, &renderables, &positions).iter() {
                render_into_viewport(&viewport, bgcolor, position, renderable, tcod);
            }
            for (_, renderable, position) in (&layer1, &renderables, &positions).iter() {
               render_into_viewport(&viewport, bgcolor, position, renderable, tcod);
            }
        }

        tcod.flush();
    }
}

fn main() {
    ApplicationBuilder::new(Game)
        .register::<Player>()
        .register::<Item>()
        .register::<Fov>()
        .register::<Description>()
        .register::<Inventory>()
        .register::<Health>()
        .with::<PlayerController>(PlayerController, "player_controller_system", 1)
        .with::<UiUpdater>(UiUpdater, "ui_updater_system", 2)
        .build()
        .run();

}
