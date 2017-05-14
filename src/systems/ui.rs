use specs::{ System, RunArg, Join };

use ui::{ Ui, UiData };
use game_stats::{ GameStats };
use components::player::{ Player };
use components::inventory::{ Inventory };
use components::common::{ Active, Description, Health };

pub struct UiUpdater;
unsafe impl Sync for UiUpdater {}

impl System<()> for UiUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (players, actives, descriptions, health, inventories, stats, mut ui) = arg.fetch(|w| {
            (w.read::<Player>(),
             w.read::<Active>(),
             w.read::<Description>(),
             w.read::<Health>(),
             w.read::<Inventory>(),
             w.read_resource::<GameStats>(),
             w.write_resource::<Ui>())
        });

        ui.update("time_left".into(), UiData::Text{ text: stats.time_left().to_string() });

        for (player, _, description, health) in (&players, &actives, &descriptions, &health).iter() {
            ui.update("active_player".into(), UiData::MultiLine { text: vec![
                description.name.clone(),
                health.health.to_string()
            ]});
        }
        for (player, _, description, health) in (&players, !&actives, &descriptions, &health).iter() {
            ui.update("inactive_player".into(), UiData::MultiLine { text: vec![
                description.name.clone(),
                health.health.to_string()
            ]});
        }
        for (player, _, inventory) in (&players, &actives, &inventories).iter() {
            ui.update("inventory".into(), UiData::MultiLine {
                text: inventory.items.iter()
                    .filter_map(|item| descriptions.get(*item))
                    .map(|description| description.name.clone())
                    .collect()
            });
        }
    }
}
