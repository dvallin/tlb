use specs::{ System, RunArg, Join };

use components::common::{ CharacterStats };
use components::inventory::{ Inventory };
use components::space::{ Position };

use event_log::{ EventLog, LogEvent };
use maps::{ Map, Maps };

pub struct StatsUpdater;
unsafe impl Sync for StatsUpdater {}

impl System<()> for StatsUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, mut char_stats, mut positions, mut inventories, mut log, mut maps) = arg.fetch(|w| {
            (w.entities(),
             w.write::<CharacterStats>(),
             w.write::<Position>(),
             w.write::<Inventory>(),
             w.write_resource::<EventLog>(),
             w.write_resource::<Maps>())
        });

        let mut graveyard = vec![];
        for (id, stats, pos) in (&entities, &char_stats, &positions).iter() {
            if stats.health == 0.0 {
                graveyard.push((id, *pos));
                log.log(LogEvent::Died(id));
            }
        }

        for (id, pos) in graveyard {
            let p = (pos.x as i32, pos.y as i32);
            maps.remove(Map::Character, &id, p);
            if let Some(inventory) = inventories.get_mut(id) {
                for item in inventory.items.iter() {
                    maps.push(Map::Item, item, p);
                    positions.insert(*item, pos);
                }
                inventory.items.clear();
            }

            char_stats.remove(id);
            positions.remove(id);
        }
    }
}
