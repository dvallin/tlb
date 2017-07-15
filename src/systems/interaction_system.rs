use specs::{ System, ReadStorage, FetchMut, Entities, WriteStorage, Join };

use components::player::{ Equipment };
use components::interaction::{ Interaction, Interactable };
use components::item::{ Item };
use components::appearance::{ Renderable };
use components::space::{ Position, Level };

use game_state::{ GameState };
use tower::{ Tower };
use maps::{ Map };

pub struct InteractionSystem;
unsafe impl Sync for InteractionSystem {}

#[derive(SystemData)]
pub struct InteractionSystemData<'a> {
    entities: Entities<'a>,
    equipments: ReadStorage<'a, Equipment>,
    items: ReadStorage<'a, Item>,
    interactions: WriteStorage<'a, Interaction>,
    interactables: WriteStorage<'a, Interactable>,
    renderables: WriteStorage<'a, Renderable>,
    positions: ReadStorage<'a, Position>,
    levels: ReadStorage<'a, Level>,
    state: FetchMut<'a, GameState>,
    tower: FetchMut<'a, Tower>,
}

impl<'a> System<'a> for InteractionSystem {
    type SystemData = InteractionSystemData<'a>;

    fn run(&mut self, mut data: InteractionSystemData) {
        let items = data.items;
        for (id, interaction, interactable) in (&*data.entities, &data.interactions,
                                                &mut data.interactables).join() {
            if let Some(equipment) = data.equipments.get(interaction.actor) {
                let active_item = equipment.active_item.and_then(|i| items.get(i));
                let passive_item = equipment.passive_item.and_then(|i| items.get(i));
                let clothing = equipment.clothing.and_then(|i| items.get(i));
                let was_sight_blocking = interactable.is_sight_blocking();
                interactable.interact_with(active_item, passive_item, clothing);
                data.renderables.insert(id, interactable.get_renderable());
                if let Some(level) = data.levels.get(id) {
                    let maps = data.tower.get_mut(level).unwrap();
                    if let Some(target_pos) = data.positions.get(id) {
                        let p = (target_pos.x as i32, target_pos.y as i32);
                        maps.set_blocking(Map::Character, &id, p, interactable.is_blocking());
                        maps.set_sight_blocking(Map::Character, &id, p, interactable.is_sight_blocking());
                        let is_sight_blocking = interactable.is_sight_blocking();
                        if was_sight_blocking != is_sight_blocking {
                            data.state.fov_needs_update = true;
                        }
                    }
                }
            }
        }
        data.interactions.clear();
    }
}
