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
mod game_state;
mod event_log;

use specs::{ World, Join, Entity };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ ApplicationBuilder };
use engine::tcod::{ Tcod };

use tcod::colors::{ self };
use tcod::input::{ KeyCode };

use maps::{ Maps, Map };
use game_stats::{ GameStats };
use game_state::{ GameState };
use event_log::{ EventLog };
use ui::{ Ui };

use components::appearance::{ Renderable, Layer0, Layer1 };
use components::space::{ Position, Spawn, Viewport };
use components::player::{ Player, Fov, Equipment };
use components::npc::{ Npc, NpcInstance };
use components::item::{ Item, ItemInstance };
use components::common::{ Active, InTurn, WaitForTurn, CharacterStats,
                          MoveToPosition, ItemStats, Description };
use components::interaction::{ Interactable, InteractableInstance, Interaction };
use components::inventory::{ Inventory };

use geometry::{ Rect };

use systems::player_controller::{ PlayerController };
use systems::interaction_system::{ InteractionSystem };
use systems::move_to_controller::{ MoveToController };
use systems::round_scheduler::{ RoundScheduler };
use systems::stats_updater::{ StatsUpdater };
use systems::ui::{ UiUpdater };

const TORCH_RADIUS: i32 = 10;
struct Game;

impl Game {
    fn create_player(&mut self, x: f32, y: f32, active: bool, name: String,
                     tcod: &mut Tcod, world: &mut World) {
        let fov_index = tcod.create_fov();
        let mut builder = world.create_now()
            .with(Player)
            .with(Spawn::for_location(x, y))
            .with(Renderable { character: '@', color: colors::WHITE })
            .with(CharacterStats { health: 100.0, max_health: 100.0 } )
            .with(Description { name: name, description: "".into() })
            .with(Fov { index: fov_index })
            .with(Inventory::new())
            .with(Equipment::new())
            .with(Layer1);
        if active {
            builder = builder.with(Active);
        }
        builder.build();
    }

    fn create_npc(&mut self, x: f32, y: f32, instance: NpcInstance, world: &mut World) -> Entity {
        let n = Npc { instance: instance };
        let builder = world.create_now()
            .with(Spawn::for_location(x, y))
            .with(components::npc::get_renderable(&n))
            .with(components::npc::get_description(&n))
            .with(components::npc::get_stats(&n))
            .with(Inventory::new())
            .with(n)
            .with(Layer1);
        builder.build()
    }

    fn create_interactable(&mut self, x: f32, y: f32, instance: InteractableInstance, world: &mut World) -> Entity {
        let interactable = Interactable::new(instance);
        let builder = world.create_now()
            .with(Spawn::for_location(x, y))
            .with(interactable.get_renderable())
            .with(interactable)
            .with(Layer0);
        builder.build()
    }

    fn create_inventory(&mut self, owner: Entity, items: Vec<ItemInstance>, world: &mut World) {
        let mut inventory = Inventory::new();
        for instance in items {
            let i = Item { instance: instance };
            let mut item = world.create_now()
                .with(Spawn::for_owner(owner))
                .with(components::item::get_renderable(&i))
                .with(components::item::get_description(&i));
            if let Some(c) = components::item::get_stats(&i) {
                item = item.with(c)
            }
            item = item.with(i).with(Layer0);
            inventory.items.push(item.build());
        }
        world.write().insert(owner, inventory);
    }

    fn create_item(&mut self, x: f32, y: f32, instance: ItemInstance, world: &mut World) {
        let i = Item { instance: instance };
        let mut builder = world.create_now()
            .with(Spawn::for_location(x, y))
            .with(components::item::get_renderable(&i))
            .with(components::item::get_description(&i));
        if let Some(c) = components::item::get_stats(&i) {
            builder = builder.with(c)
        }
        builder
            .with(i)
            .with(Layer0)
            .build();
    }

    fn reset_world(&mut self, tcod: &mut Tcod, world: &mut World) {
        let entities = world.entities();
        let mut stats = world.write_resource::<GameStats>();
        let mut positions = world.write::<Position>();
        let mut inventories = world.write::<Inventory>();
        let mut char_stats = world.write::<CharacterStats>();
        let mut interactables = world.write::<Interactable>();
        let mut maps = world.write_resource::<Maps>();
        let mut state = world.write_resource::<GameState>();

        let mut in_turns = world.write::<InTurn>();
        let mut waits = world.write::<WaitForTurn>();
        let mut moves = world.write::<MoveToPosition>();

        let spawns = world.read::<Spawn>();
        let fovs = world.read::<Fov>();
        let players = world.read::<Player>();
        let npcs = world.read::<Npc>();
        let items = world.read::<Item>();

        state.reset();

        in_turns.clear();
        waits.clear();
        moves.clear();

        maps.clear_all();
        for (id, spawn) in (&entities, &spawns).iter() {
            if let Some(loc) = spawn.location {
                positions.insert(id, Position { x: loc.0, y: loc.1 });
            } else if let Some(owner) = spawn.owner {
                if positions.remove(id).is_some() {
                    if let Some(inventory) = inventories.get_mut(owner) {
                        inventory.push(id);
                    }
                }
            }
        }
        for (_, interactable) in (&entities, &mut interactables).iter() {
            interactable.reset();
        }

        for (id, interactable, pos) in (&entities, &mut interactables, &mut positions).iter() {
            let p = (pos.x as i32, pos.y as i32);
            maps.push(Map::Character, &id, p);
            maps.set_blocking(Map::Character, &id, p, interactable.is_blocking());
            maps.set_sight_blocking(Map::Character, &id, p, interactable.is_sight_blocking());
        }

        for (id, _, pos) in (&entities, &players, &mut positions).iter() {
            maps.push(Map::Character, &id, (pos.x as i32, pos.y as i32));
        }
        for (id, _, pos, stats) in (&entities, &npcs, &mut positions, &mut char_stats).iter() {
            maps.push(Map::Character, &id, (pos.x as i32, pos.y as i32));
            stats.reset();
        }
        for (id, _, pos) in (&entities, &items, &mut positions).iter() {
            maps.push(Map::Item, &id, (pos.x as i32, pos.y as i32));
        }

        for fov in (&fovs).iter() {
            tcod.initialize_fov(fov.index, &maps);
        }

        stats.reset();
    }
}

fn render_into_viewport(viewport: &Viewport, position: &Position, renderable: &Renderable, tcod: &mut Tcod) {
    let p = (position.x as i32, position.y as i32);
    if viewport.visible(p) && tcod.is_in_fov(p) {
        let pos = viewport.transform(p);
        tcod.render_character(pos, renderable.color, renderable.character);
    }
}

impl State for Game {
    fn start(&mut self, tcod: &mut Tcod, world: &mut World) {
        world.add_resource::<InputHandler>(InputHandler::default());
        world.add_resource::<GameStats>(GameStats::default());
        world.add_resource::<GameState>(GameState::default());
        world.add_resource::<EventLog>(EventLog::default());
        world.add_resource::<Viewport>(Viewport::new(15, 15, 80, 40));
        let mut maps = Maps::new();
        maps.build();
        world.add_resource::<Maps>(maps);

        self.create_player(15.0, 15.0, true, "Colton".into(), tcod, world);
        self.create_player(16.0, 16.0, false, "Gage".into(), tcod, world);

        self.create_interactable(25.0, 21.0, InteractableInstance::KeyDoor(3, false), world);

        {
            let guard = self.create_npc(31.0, 24.0, NpcInstance::Guard, world);
            self.create_inventory(guard, vec![ItemInstance::FlickKnife,
                                              ItemInstance::Watch,
                                              ItemInstance::KeyCard(3)], world);
        }
        self.create_npc(29.0, 24.0, NpcInstance::Technician, world);
        self.create_npc(31.0, 29.0, NpcInstance::Accountant, world);

        self.create_item(14.0, 15.0, ItemInstance::FlickKnife, world);
        self.create_item(13.0, 15.0, ItemInstance::DartGun, world);
        self.create_item(33.0, 25.0, ItemInstance::Simstim, world);
        self.create_item(23.0, 25.0, ItemInstance::HitachiRam, world);
        self.create_item(28.0, 21.0, ItemInstance::Shuriken, world);

        let mut ui = Ui::new();
        ui.add("active_player".into(), Rect::new(1, 1, 11, 2));
        ui.add("time_left".into(), Rect::new(37, 1, 5, 1));
        ui.add("inventory".into(), Rect::new(44, 1, 21, 5));
        ui.add("event_log".into(), Rect::new(14, 1, 21, 5));
        ui.add("inactive_player".into(), Rect::new(67, 1, 11, 2));
        world.add_resource::<Ui>(ui);

        self.reset_world(tcod, world);
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
            self.reset_world(tcod, world);
        }

        let fovs = world.read::<Fov>();
        let positions = world.read::<Position>();
        for (fov, position) in (&fovs, &positions).iter() {
            tcod.compute_fov(fov.index, (position.x as i32, position.y as i32), TORCH_RADIUS);
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
        .register::<InTurn>()
        .register::<Interactable>()
        .register::<Interaction>()
        .register::<WaitForTurn>()
        .register::<Inventory>()
        .register::<Equipment>()
        .register::<CharacterStats>()
        .register::<ItemStats>()
        .register::<MoveToPosition>()
        .with::<PlayerController>(PlayerController, "player_controller_system", 1)
        .with::<MoveToController>(MoveToController, "move_to_controller_system", 1)
        .with::<InteractionSystem>(InteractionSystem, "interaction_system", 1)
        .with::<RoundScheduler>(RoundScheduler, "round_scheduler_system", 2)
        .with::<StatsUpdater>(StatsUpdater, "stats_updater_system", 2)
        .with::<UiUpdater>(UiUpdater, "ui_updater_system", 2)
        .build()
        .run();

}
