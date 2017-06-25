use std::collections::VecDeque;
use specs::{ Entity };

pub enum LogEvent {
    FinishedTurn(Entity),
}


pub struct EventLog {
    pub logs: VecDeque<LogEvent>,
}

impl Default for EventLog {
    fn default() -> Self {
        EventLog { logs: VecDeque::new() }
    }
}

impl EventLog {
    pub fn log(&mut self, event: LogEvent) {
        self.logs.push_front(event);
    }
}
