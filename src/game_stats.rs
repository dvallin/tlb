use std::time::{ Duration };
use engine::time::{ Timer };

pub struct GameStats {
    timer: Timer,
    time_loop_length: Duration,
}

impl Default for GameStats {
    fn default() -> Self {
        let time_loop_length = Duration::from_secs(10 * 60);
        let timer = Timer::new(time_loop_length);
        GameStats { timer: timer, time_loop_length: time_loop_length }
    }
}

impl GameStats {
    pub fn time_left(&self) -> i32 {
        return self.timer.time_left().as_secs() as i32;
    }

    pub fn reset(&mut self) {
        self.timer = Timer::new(self.time_loop_length);
    }
}
