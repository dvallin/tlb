use engine::time::{ Stopwatch };

pub struct GameStats {
    timer: Stopwatch,
}

impl Default for GameStats {
    fn default() -> Self {
        let mut timer = Stopwatch::default();
        timer.start();
        GameStats { timer: timer }
    }
}

impl GameStats {
    pub fn time_left(&self) -> i32 {
        return (10 * 60) - self.timer.elapsed().as_secs() as i32;
    }

    pub fn reset(&mut self) {
        self.timer.restart();
    }
}
