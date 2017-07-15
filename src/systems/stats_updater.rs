use specs::{ System, ReadStorage, FetchMut, Entities, WriteStorage, Join };

use components::common::{ CharacterStats };
use components::inventory::{ Inventory };
use components::space::{ Position, Level };

use event_log::{ EventLog, LogEvent };
use tower::{ Tower };
use maps::{ Map };

pub struct StatsUpdater;
unsafe impl Sync for StatsUpdater {}

#[derive(SystemData)]
pub struct StatsUpdaterData<'a> {
    entities: Entities<'a>,
    char_stats: WriteStorage<'a, CharacterStats>,
    positions: WriteStorage<'a, Position>,
    levels: ReadStorage<'a, Level>,
    inventories: WriteStorage<'a, Inventory>,
    log: FetchMut<'a, EventLog>,
    tower: FetchMut<'a, Tower>,
}

impl<'a> System<'a> for StatsUpdater {
    type SystemData = StatsUpdaterData<'a>;

    fn run(&mut self, mut data: StatsUpdaterData) {
        let mut graveyard = vec![];
        for (id, stats, pos, level) in (&*data.entities, &data.char_stats, &data.positions, &data.levels).join() {
            if stats.health <= 0.0 {
                graveyard.push((id, *pos, level));
                data.log.log(LogEvent::Died(id));
            }
        }

        for (id, pos, level) in graveyard {
            let p = (pos.x as i32, pos.y as i32);
            let maps = data.tower.get_mut(level).unwrap();
            maps.remove(Map::Character, &id, p);
            if let Some(inventory) = data.inventories.get_mut(id) {
                for item in inventory.items.iter() {
                    maps.push(Map::Item, item, p);
                    data.positions.insert(*item, pos);
                }
                inventory.items.clear();
            }

            data.char_stats.remove(id);
            data.positions.remove(id);
        }
    }
}
