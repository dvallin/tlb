use std::time::{ Duration, Instant };

pub struct Time {
    pub delta_time: Duration,
    pub fixed_step: Duration,
    pub last_fixed_update: Instant,
}

pub enum Stopwatch {
    Waiting,
    Started(Duration, Instant),
    Ended(Duration),
}

impl Default for Stopwatch {
    fn default() -> Self {
        Stopwatch::Waiting
    }
}

impl Stopwatch {
    pub fn elapsed(&self) -> Duration {
        match *self {
            Stopwatch::Waiting => Duration::new(0, 0),
            Stopwatch::Started(duration, start) => duration + start.elapsed(),
            Stopwatch::Ended(duration) => duration,
        }
    }

    pub fn start(&mut self) {
        match self {
            &mut Stopwatch::Waiting => self.restart(),
            &mut Stopwatch::Ended(duration) => {
                *self = Stopwatch::Started(duration, Instant::now())
            },
            _ => {}
        }
    }

    pub fn restart(&mut self) {
        *self = Stopwatch::Started(Duration::new(0, 0), Instant::now())
    }

    pub fn stop(&mut self) {
        if let &mut Stopwatch::Started(duration, start) = self {
            *self = Stopwatch::Ended(duration + start.elapsed());
        }
    }
}
