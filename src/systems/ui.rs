use specs::{ System, RunArg, Join };

use ui::{ Ui, UiData };
use game_stats::{ GameStats };
use components::player::{ Player };
use components::inventory::{ Inventory };
use components::common::{ Description, Health };

pub struct UiUpdater;
unsafe impl Sync for UiUpdater {}

impl System<()> for UiUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (players, descriptions, health, inventories, stats, mut ui) = arg.fetch(|w| {
            (w.read::<Player>(),
             w.read::<Description>(),
             w.read::<Health>(),
             w.read::<Inventory>(),
             w.read_resource::<GameStats>(),
             w.write_resource::<Ui>())
        });

        ui.update("time_left".into(), UiData::Text{ text: stats.time_left().to_string() });

        for (player, description, health) in (&players, &descriptions, &health).iter() {
            if player.active {
                ui.update("active_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    health.health.to_string()
                ]});

            } else {
                ui.update("inactive_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    health.health.to_string()
                ]});
            }
        }
        for (player, inventory) in (&players, &inventories).iter() {
            if player.active {
                ui.update("inventory".into(), UiData::MultiLine {
                    text: inventory.items.iter()
                        .filter_map(|item| descriptions.get(*item))
                        .map(|description| description.name.clone())
                        .collect()
                });
            }
        }
    }
}
