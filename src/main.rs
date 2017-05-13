extern crate tcod;
extern crate specs;
extern crate num_cpus;

mod engine;
mod components;
mod tilemap;
mod ui;
mod geometry;
mod systems;
mod items;
mod itemmap;

use specs::{ World, Join };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ ApplicationBuilder };
use engine::tcod::{ Tcod };

use tcod::colors::{ self };

use tilemap::{ TileMap };
use ui::{ Ui };

use components::appearance::{ Renderable, Layer0, Layer1 };
use components::space::{ Position, Viewport };
use components::player::{ Player, Fov };
use components::common::{ Health, Description };
use components::inventory::{ Inventory };

use geometry::{ Pos, Rect };

use systems::player_controller::{ PlayerController };
use systems::ui::{ UiUpdater };

use itemmap::{ Item, ItemMap };

const TORCH_RADIUS: i32 = 10;
struct Game;

impl Game {
    fn create_player(&mut self, index: usize, x: f32, y: f32, active: bool, name: String,
                     tcod: &mut Tcod, world: &mut World) {
        let fov_index = tcod.create_fov();
        tcod.initialize_fov(fov_index, world);
        world.create_now()
            .with(Position { x: x, y: y })
            .with(Renderable { character: '@', color: colors::WHITE })
            .with(Player { index: index, active: active })
            .with(Health { health: 100.0 } )
            .with(Description { name: name, description: "".into() })
            .with(Fov { index: fov_index })
            .with(Inventory::new())
            .with(Layer1)
            .build();
    }

    fn create_item(&mut self, x: f32, y: f32, map: &mut ItemMap, item: Item, world: &mut World) {
        let entity = world.create_now()
            .with(Position { x: x, y: y })
            .with(itemmap::get_renderable(&item))
            .with(itemmap::get_description(&item))
            .with(Layer0)
            .with(item)
            .build();

        map.push(&entity, x as i32, y as i32);
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

        let mut tile_map = TileMap::new();
        tile_map.build();
        world.add_resource::<TileMap>(tile_map);

        let mut item_map = ItemMap::new();
        self.create_item(14.0, 15.0, &mut item_map, Item::FlickKnife, world);
        world.add_resource::<ItemMap>(item_map);

        let mut ui = Ui::new();
        ui.add("active_player".into(), Rect::new(1, 1, 11, 2));
        ui.add("time".into(), Rect::new(34, 1, 11, 1));
        ui.add("inactive_player".into(), Rect::new(57, 1, 11, 2));
        world.add_resource::<Ui>(ui);

        self.create_player(1, 15.0, 15.0, true, "Colton".into(), tcod, world);
        self.create_player(2, 16.0, 16.0, false, "Gage".into(), tcod, world);


        let viewport = Viewport::new(15, 15, 80, 40);
        world.add_resource::<Viewport>(viewport);
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
