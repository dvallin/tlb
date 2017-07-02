use std::slice::Iter;
use specs::{ Entity };

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Entry(pub Entity, pub bool, pub bool);

pub struct EntityMap {
    map: Vec<Vec<Vec<Entry>>>,
    width: i32,
    height: i32,
}

impl EntityMap {
    pub fn new() -> Self {
        let map = vec![vec![vec![]; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        EntityMap {
            map: map,
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        }
    }

    pub fn clear(&mut self) {
        self.map = vec![vec![vec![]; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    }

    pub fn remove(&mut self, entity: &Entity, p: (i32, i32)) -> Option<Entry> {
        self.map[p.0 as usize][p.1 as usize].iter()
            .position(|e| e.0 == *entity)
            .map(|index| self.map[p.0 as usize][p.1 as usize].swap_remove(index))
    }

    pub fn set_blocking(&mut self, entity: &Entity, p: (i32, i32), blocking: bool) {
        let entry = self.map[p.0 as usize][p.1 as usize].iter_mut()
            .filter(|e| e.0 == *entity)
            .next();
        if let Some(e) = entry {
            e.1 = blocking;
        }
    }

    pub fn set_sight_blocking(&mut self, entity: &Entity, p: (i32, i32), blocking: bool) {
        let entry = self.map[p.0 as usize][p.1 as usize].iter_mut()
            .filter(|e| e.0 == *entity)
            .next();
        if let Some(e) = entry {
            e.2 = blocking;
        }
    }

    pub fn is_empty(&self, p: (i32, i32)) -> bool {
        self.map[p.0 as usize][p.1 as usize].is_empty()
    }

    pub fn push(&mut self, entity: &Entity, p: (i32, i32)) {
        self.map[p.0 as usize][p.1 as usize].push(Entry(*entity, true, false));
    }

    pub fn push_entry(&mut self, entity: Entry, p: (i32, i32)) {
        self.map[p.0 as usize][p.1 as usize].push(entity);
    }

    pub fn pop(&mut self, p: (i32, i32)) -> Option<Entry> {
        self.map[p.0 as usize][p.1 as usize].pop()
    }

    pub fn get(&self, p: (i32, i32)) -> &Vec<Entry> {
        &self.map[p.0 as usize][p.1 as usize]
    }
}
