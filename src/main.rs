extern crate tcod;
extern crate specs;
extern crate rayon;
extern crate shred;
#[macro_use]
extern crate shred_derive;

mod engine;
mod components;
mod tile_map;
mod entity_map;
mod ui;
mod geometry;
mod systems;
mod tower;
mod maps;
mod game_stats;
mod game_state;
mod event_log;

use specs::{ World, Join, DispatcherBuilder };

use engine::state::{ State, Transition };
use engine::input_handler::{ InputHandler };
use engine::application::{ Application };
use engine::tcod::{ Tcod };

use tcod::colors::{ self };
use tcod::input::{ KeyCode };

use tower::{ Tower };
use maps::{ Map };
use game_stats::{ GameStats };
use game_state::{ GameState };
use event_log::{ EventLog };
use ui::{ Ui };

use components::appearance::{ Renderable, Layer0, Layer1 };
use components::space::{ Position, Spawn, Viewport, Level };
use components::player::{ Player, Fov, Equipment };
use components::npc::{ Npc };
use components::item::{ Item };
use components::common::{ Active, InTurn, WaitForTurn, CharacterStats,
                          MoveToPosition, ItemStats, Description };
use components::interaction::{ Interactable, Interaction };
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

    fn reset_world(&mut self, tcod: &mut Tcod, world: &mut World) {
        let entities = world.entities();
        let mut stats = world.write_resource::<GameStats>();
        let mut positions = world.write::<Position>();
        let mut levels = world.write::<Level>();
        let mut inventories = world.write::<Inventory>();
        let mut char_stats = world.write::<CharacterStats>();
        let mut interactables = world.write::<Interactable>();
        let mut tower = world.write_resource::<Tower>();
        let mut state = world.write_resource::<GameState>();

        let mut in_turns = world.write::<InTurn>();
        let mut waits = world.write::<WaitForTurn>();
        let mut moves = world.write::<MoveToPosition>();

        let spawns = world.read::<Spawn>();
        let players = world.read::<Player>();
        let npcs = world.read::<Npc>();
        let items = world.read::<Item>();

        state.reset();

        in_turns.clear();
        waits.clear();
        moves.clear();

        tower.clear();
        for (id, spawn) in (&*entities, &spawns).join() {
            if let Some(loc) = spawn.location {
                positions.insert(id, Position { x: loc.0, y: loc.1 });
                levels.insert(id, loc.2);
            } else if let Some(owner) = spawn.owner {
                if positions.remove(id).is_some() {
                    levels.remove(id);
                    if let Some(inventory) = inventories.get_mut(owner) {
                        inventory.push(id);
                    }
                }
            }
        }
        for (_, interactable) in (&*entities, &mut interactables).join() {
            interactable.reset();
        }

        for (id, interactable, pos, level) in (&*entities, &mut interactables, &mut positions, &mut levels).join() {
            let p = (pos.x as i32, pos.y as i32);
            if let Some(maps) = tower.get_mut(level) {
                maps.push(Map::Character, &id, p);
                maps.set_blocking(Map::Character, &id, p, interactable.is_blocking());
                maps.set_sight_blocking(Map::Character, &id, p, interactable.is_sight_blocking());
            }
        }

        for (id, _, pos, level) in (&*entities, &players, &mut positions, &mut levels).join() {
            if let Some(maps) = tower.get_mut(level) {
                maps.push(Map::Character, &id, (pos.x as i32, pos.y as i32));
            }
        }
        for (id, _, pos, stats, level) in (&*entities, &npcs, &mut positions, &mut char_stats, &mut levels).join() {
            if let Some(maps) = tower.get_mut(level) {
                maps.push(Map::Character, &id, (pos.x as i32, pos.y as i32));
            }
            stats.reset();
        }
        for (id, _, pos, level) in (&*entities, &items, &mut positions, &mut levels).join() {
            if let Some(maps) = tower.get_mut(level) {
                maps.push(Map::Item, &id, (pos.x as i32, pos.y as i32));
            }
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

        let mut tower = Tower::new(&[Level::Tower(0)]);
        tower.build(tcod, world);
        world.add_resource::<Tower>(tower);

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

        let state = world.read_resource::<GameState>();
        let fovs = world.read::<Fov>();
        let positions = world.read::<Position>();
        let levels = world.read::<Level>();

        if state.fov_needs_update {
            let tower = world.read_resource::<Tower>();
            for (fov, level) in (&fovs, &levels).join() {
                tcod.update_fov(*fov.fov_map.get(level).unwrap(),
                                tower.get(level).unwrap());
            }
        }
        for (fov, position, level) in (&fovs, &positions, &levels).join() {
            tcod.compute_fov(*fov.fov_map.get(level).unwrap(),
                             (position.x as i32, position.y as i32), TORCH_RADIUS);
        }

        let mut tower = world.write_resource::<Tower>();
        tower.update(tcod);

        Transition::None
    }

    fn render(&mut self, tcod: &mut Tcod, world: &mut World) {
        let renderables = world.read::<Renderable>();
        let positions = world.read::<Position>();
        let layer0 = world.read::<Layer0>();
        let layer1 = world.read::<Layer1>();
        let actives = world.read::<Active>();
        let levels = world.read::<Level>();
        let tower = world.read_resource::<Tower>();
        let ui = world.read_resource::<Ui>();
        let viewport = world.read_resource::<Viewport>();

        tcod.clear(colors::BLACK);

        {
            if let Some((level, _)) = (&levels, &actives).join().next() {
                tower.draw(level, tcod, &viewport);
            }
            ui.draw(tcod);

            for (_, renderable, position) in (&layer0, &renderables, &positions).join() {
                render_into_viewport(&viewport, position, renderable, tcod);
            }
            for (_, renderable, position) in (&layer1, &renderables, &positions).join() {
               render_into_viewport(&viewport, position, renderable, tcod);
            }
        }

        tcod.flush();
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Player>();
    world.register::<Level>();
    world.register::<Npc>();
    world.register::<Spawn>();
    world.register::<Item>();
    world.register::<Fov>();
    world.register::<Description>();
    world.register::<Active>();
    world.register::<InTurn>();
    world.register::<Interactable>();
    world.register::<Interaction>();
    world.register::<WaitForTurn>();
    world.register::<Inventory>();
    world.register::<Equipment>();
    world.register::<CharacterStats>();
    world.register::<ItemStats>();
    world.register::<MoveToPosition>();

    let dispatcher = DispatcherBuilder::new()
        .add(PlayerController, "player_controller_system", &[])
        .add(MoveToController, "move_to_controller", &[])
        .add(InteractionSystem, "interaction_system", &[])
        .add(RoundScheduler, "round_scheduler", &[])
        .add(StatsUpdater, "stats_updater", &[])
        .add(UiUpdater, "ui_updater", &[]);
    Application::new(Game, world, dispatcher.build()).run();
}
