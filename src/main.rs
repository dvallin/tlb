extern crate tcod;
extern crate specs;
extern crate num_cpus;

mod engine;
mod components;
mod tile_map;
mod entity_map;
mod ui;
mod geometry;
mod systems;
mod maps;
mod game_stats;

use specs::{ World, Join };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ ApplicationBuilder };
use engine::tcod::{ Tcod };

use tcod::colors::{ self };
use tcod::input::{ KeyCode };

use maps::{ Maps, Map };
use game_stats::{ GameStats };
use ui::{ Ui };

use components::appearance::{ Renderable, Layer0, Layer1 };
use components::space::{ Position, Spawn, Viewport };
use components::player::{ Player, Fov };
use components::npc::{ Npc, NpcInstance };
use components::item::{ Item, ItemInstance };
use components::common::{ Active, MoveToPosition, Health, Description };
use components::inventory::{ Inventory };

use geometry::{ Pos, Rect };

use systems::player_controller::{ PlayerController };
use systems::move_to_controller::{ MoveToController };
use systems::round_scheduler::{ RoundScheduler };
use systems::ui::{ UiUpdater };

const TORCH_RADIUS: i32 = 10;
struct Game;

impl Game {
    fn create_player(&mut self, x: f32, y: f32, active: bool, name: String,
                     tcod: &mut Tcod, world: &mut World) {
        let fov_index = tcod.create_fov();
        tcod.initialize_fov(fov_index, world);

        let mut builder = world.create_now()
            .with(Player)
            .with(Spawn { x: x, y: y })
            .with(Renderable { character: '@', color: colors::WHITE })
            .with(Health { health: 100.0 } )
            .with(Description { name: name, description: "".into() })
            .with(Fov { index: fov_index })
            .with(Inventory::new())
            .with(Layer1);
        if active {
            builder = builder.with(Active);
        }
        builder.build();
    }

    fn create_npc(&mut self, x: f32, y: f32, instance: NpcInstance, world: &mut World) {
        let n = Npc { instance: instance };
        world.create_now()
            .with(Spawn { x: x, y: y })
            .with(components::npc::get_renderable(&n))
            .with(components::npc::get_description(&n))
            .with(components::npc::get_health(&n))
            .with(components::npc::get_inventory(&n))
            .with(n)
            .with(Layer1)
            .build();
    }

    fn create_item(&mut self, x: f32, y: f32, instance: ItemInstance, world: &mut World) {
        let i = Item { instance: instance };
        world.create_now()
            .with(Spawn { x: x, y: y })
            .with(components::item::get_renderable(&i))
            .with(components::item::get_description(&i))
            .with(i)
            .with(Layer0)
            .build();
    }

    fn reset_world(&mut self, world: &mut World) {
        let entities = world.entities();
        let mut stats = world.write_resource::<GameStats>();
        let mut positions = world.write::<Position>();
        let mut maps = world.write_resource::<Maps>();

        let spawns = world.read::<Spawn>();
        let players = world.read::<Player>();
        let npcs = world.read::<Npc>();
        let items = world.read::<Item>();

        maps.clear_all();
        for (id, spawn) in (&entities, &spawns).iter() {
            positions.insert(id, Position { x: spawn.x, y: spawn.y });
        }

        for (id, _, pos) in (&entities, &players, &mut positions).iter() {
            maps.push(Map::Character, &id, pos.x as i32, pos.y as i32);
        }
        for (id, _, pos) in (&entities, &npcs, &mut positions).iter() {
            maps.push(Map::Character, &id, pos.x as i32, pos.y as i32);
        }
        for (id, _, pos) in (&entities, &items, &mut positions).iter() {
            maps.push(Map::Item, &id, pos.x as i32, pos.y as i32);
        }

        stats.reset();
    }
}

fn render_into_viewport(viewport: &Viewport, position: &Position, renderable: &Renderable, tcod: &mut Tcod) {
    let p = Pos { x: position.x as i32, y: position.y as i32 };
    if viewport.visible(p) && tcod.is_in_fov(p.x, p.y) {
        let pos = viewport.transform(p);
        tcod.render_character(pos.x, pos.y, renderable.color, renderable.character);
    }
}

impl State for Game {
    fn start(&mut self, tcod: &mut Tcod, world: &mut World) {
        world.add_resource::<InputHandler>(InputHandler::default());
        world.add_resource::<GameStats>(GameStats::default());
        world.add_resource::<Viewport>(Viewport::new(15, 15, 80, 40));
        let mut maps = Maps::new();
        maps.build();
        world.add_resource::<Maps>(maps);

        self.create_player(15.0, 15.0, true, "Colton".into(), tcod, world);
        self.create_player(16.0, 16.0, false, "Gage".into(), tcod, world);

        self.create_npc(31.0, 24.0, NpcInstance::Guard, world);
        self.create_npc(29.0, 24.0, NpcInstance::Technician, world);
        self.create_npc(31.0, 29.0, NpcInstance::Accountant, world);

        self.create_item(14.0, 15.0, ItemInstance::FlickKnife, world);
        self.create_item(33.0, 25.0, ItemInstance::Simstim, world);
        self.create_item(23.0, 25.0, ItemInstance::HitachiRam, world);
        self.create_item(28.0, 21.0, ItemInstance::Shuriken, world);

        let mut ui = Ui::new();
        ui.add("active_player".into(), Rect::new(1, 1, 11, 2));
        ui.add("time_left".into(), Rect::new(24, 1, 11, 1));
        ui.add("inventory".into(), Rect::new(38, 1, 21, 5));
        ui.add("inactive_player".into(), Rect::new(67, 1, 11, 2));
        world.add_resource::<Ui>(ui);

        self.reset_world(world);
    }

    fn handle_events(&mut self, tcod: &mut Tcod, world: &mut World) -> Transition {
        let mut input = world.write_resource::<InputHandler>();
        input.update();
        if input.is_key_pressed(KeyCode::Escape) {
            return Transition::Exit
        } else if input.is_key_pressed(KeyCode::F5) {
            tcod.switch_fullscreen();
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
        for (fov, position) in (&fovs, &positions).iter() {
            tcod.compute_fov(fov.index, position.x as i32, position.y as i32, TORCH_RADIUS);
        }

        let mut tilemap = world.write_resource::<Maps>();
        tilemap.update(tcod);

        Transition::None
    }

    fn render(&mut self, tcod: &mut Tcod, world: &mut World) {
        let renderables = world.read::<Renderable>();
        let positions = world.read::<Position>();
        let layer0 = world.read::<Layer0>();
        let layer1 = world.read::<Layer1>();
        let maps = world.read_resource::<Maps>();
        let ui = world.read_resource::<Ui>();
        let viewport = world.read_resource::<Viewport>();

        tcod.clear(colors::BLACK);

        {
            maps.draw(tcod, &viewport);
            ui.draw(tcod);

            for (_, renderable, position) in (&layer0, &renderables, &positions).iter() {
                render_into_viewport(&viewport, position, renderable, tcod);
            }
            for (_, renderable, position) in (&layer1, &renderables, &positions).iter() {
               render_into_viewport(&viewport, position, renderable, tcod);
            }
        }

        tcod.flush();
    }
}

fn main() {
    ApplicationBuilder::new(Game)
        .register::<Player>()
        .register::<Npc>()
        .register::<Spawn>()
        .register::<Item>()
        .register::<Fov>()
        .register::<Description>()
        .register::<Active>()
        .register::<Inventory>()
        .register::<Health>()
        .register::<MoveToPosition>()
        .with::<PlayerController>(PlayerController, "player_controller_system", 1)
        .with::<MoveToController>(MoveToController, "move_to_controller_system", 1)
        .with::<RoundScheduler>(RoundScheduler, "round_scheduler_system", 1)
        .with::<UiUpdater>(UiUpdater, "ui_updater_system", 2)
        .build()
        .run();

}
