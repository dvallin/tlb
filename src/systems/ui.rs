use specs::{ System, RunArg, Join };

use engine::time::{ Time };
use ui::{ Ui, UiData };
use components::player::{ Player };
use components::common::{ Description, Health };

pub struct UiUpdater;
unsafe impl Sync for UiUpdater {}

impl System<()> for UiUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (players, descriptions, health, time, mut ui) = arg.fetch(|w| {
            (w.read::<Player>(),
             w.read::<Description>(),
             w.read::<Health>(),
             w.read_resource::<Time>(),
             w.write_resource::<Ui>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;
        ui.update("time".into(), UiData::Text{ text: delta_time.to_string() });

        for (player, description, stats) in (&players, &descriptions, &health).iter() {
            if player.active {
                ui.update("active_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    stats.health.to_string()
                ]});
            } else {
                ui.update("inactive_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    stats.health.to_string()
                ]});
            }
        }
    }
}
