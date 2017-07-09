use specs::{ System, RunArg, Join };

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

impl System<()> for InteractionSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, equipments, items, mut interactions,
             mut interactables, mut renderables, positions, levels, mut state, mut tower) = arg.fetch(|w| {
            (w.entities(),
             w.read::<Equipment>(),
             w.read::<Item>(),
             w.write::<Interaction>(),
             w.write::<Interactable>(),
             w.write::<Renderable>(),
             w.read::<Position>(),
             w.read::<Level>(),
             w.write_resource::<GameState>(),
             w.write_resource::<Tower>())
        });

        for (id, interaction, interactable) in (&entities, &interactions, &mut interactables).iter() {
            if let Some(equipment) = equipments.get(interaction.actor) {
                let active_item = equipment.active_item.and_then(|i| items.get(i));
                let passive_item = equipment.passive_item.and_then(|i| items.get(i));
                let clothing = equipment.clothing.and_then(|i| items.get(i));
                let was_sight_blocking = interactable.is_sight_blocking();
                interactable.interact_with(active_item, passive_item, clothing);
                renderables.insert(id, interactable.get_renderable());
                if let Some(level) = levels.get(id) {
                    let maps = tower.get_mut(level).unwrap();
                    if let Some(target_pos) = positions.get(id) {
                        let p = (target_pos.x as i32, target_pos.y as i32);
                        maps.set_blocking(Map::Character, &id, p, interactable.is_blocking());
                        maps.set_sight_blocking(Map::Character, &id, p, interactable.is_sight_blocking());
                        let is_sight_blocking = interactable.is_sight_blocking();
                        if was_sight_blocking != is_sight_blocking {
                            state.fov_needs_update = true;
                        }
                    }
                }
            }
        }
        interactions.clear();
    }
}
