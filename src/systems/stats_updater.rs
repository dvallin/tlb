use specs::{ System, RunArg, Join };

use components::common::{ CharacterStats };
use components::inventory::{ Inventory };
use components::space::{ Position, Level };

use event_log::{ EventLog, LogEvent };
use tower::{ Tower };
use maps::{ Map };

pub struct StatsUpdater;
unsafe impl Sync for StatsUpdater {}

impl System<()> for StatsUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, mut char_stats, mut positions, levels, mut inventories, mut log, mut tower) = arg.fetch(|w| {
            (w.entities(),
             w.write::<CharacterStats>(),
             w.write::<Position>(),
             w.read::<Level>(),
             w.write::<Inventory>(),
             w.write_resource::<EventLog>(),
             w.write_resource::<Tower>())
        });

        let mut graveyard = vec![];
        for (id, stats, pos, level) in (&entities, &char_stats, &positions, &levels).iter() {
            if stats.health <= 0.0 {
                graveyard.push((id, *pos, level));
                log.log(LogEvent::Died(id));
            }
        }

        for (id, pos, level) in graveyard {
            let p = (pos.x as i32, pos.y as i32);
            let maps = tower.get_mut(level).unwrap();
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
