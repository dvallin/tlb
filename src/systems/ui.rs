use specs::{ System, RunArg, Join };

use ui::{ Ui, UiData };
use game_stats::{ GameStats };
use components::player::{ Player };
use components::space::{ Position, Viewport };
use components::inventory::{ Inventory };
use engine::input_handler::{ InputHandler };
use components::common::{ Active, InTurn, InTurnState, Description, Health };

use maps::{ Map, Maps };

pub struct UiUpdater;
unsafe impl Sync for UiUpdater {}

impl System<()> for UiUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, positions, actives, in_turns, descriptions, health, inventories,
             input, stats, viewport, mut maps, mut ui) = arg.fetch(|w| {
             (w.entities(),
              w.read::<Player>(),
              w.read::<Position>(),
              w.read::<Active>(),
              w.read::<InTurn>(),
              w.read::<Description>(),
              w.read::<Health>(),
              w.read::<Inventory>(),
              w.read_resource::<InputHandler>(),
              w.read_resource::<GameStats>(),
              w.read_resource::<Viewport>(),
              w.write_resource::<Maps>(),
              w.write_resource::<Ui>())
        });

        maps.clear_highlights();

        ui.update("time_left".into(), UiData::Text{ text: stats.time_left().to_string() });

        for (id, _, p, description, health, inventory) in (&entities, &players, &positions, &descriptions, &health, &inventories).iter() {
            let active = actives.get(id);
            let in_turn = in_turns.get(id);

            if active.is_some() || active.is_some() {
                ui.update("active_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    health.health.to_string()
                ]});
                ui.update("inventory".into(), UiData::MultiLine {
                    text: inventory.items.iter()
                        .filter_map(|item| descriptions.get(*item))
                        .map(|description| description.name.clone())
                        .collect()
                });
            }

            if active.is_none() && in_turn.is_none() {
                ui.update("inactive_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    health.health.to_string()
                ]});
            }

            if let Some(turn) = in_turn {
                match turn.0 {
                    InTurnState::Idle => {
                        let pos_trans = viewport.inv_transform(input.mouse_pos);
                        if let Some(pos) = maps.screen_to_map(pos_trans) {
                            if viewport.visible(pos) {
                                let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                                maps.add_highlights(path);
                            }
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}
