use std::slice::Iter;
use specs::{ Entity };

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

pub struct EntityMap {
    map: Vec<Vec<Option<Entity>>>,
    width: i32,
    height: i32,
}
impl EntityMap {
    pub fn new() -> Self {
        let map = vec![vec![None; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        EntityMap {
            map: map,
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        }
    }

    pub fn clear(&mut self) {
        self.map = vec![vec![None; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    }

    pub fn remove(&mut self, _: &Entity, p: (i32, i32)) -> Option<Entity> {
        self.pop(p)
    }

    pub fn is_empty(&self, p: (i32, i32)) -> bool {
        self.map[p.0 as usize][p.1 as usize].is_none()
    }

    pub fn push(&mut self, entity: &Entity, p: (i32, i32)) {
        self.map[p.0 as usize][p.1 as usize] = Some(*entity);
    }

    pub fn pop(&mut self, p: (i32, i32)) -> Option<Entity> {
        let e = self.map[p.0 as usize][p.1 as usize];
        self.map[p.0 as usize][p.1 as usize] = None;
        e
    }

    pub fn get(&self, p: (i32, i32)) -> Option<Entity> {
        self.map[p.0 as usize][p.1 as usize]
    }
}

pub struct EntityStackMap {
    map: Vec<Vec<Vec<Entity>>>,
    width: i32,
    height: i32,
}

impl EntityStackMap {
    pub fn new() -> Self {
        let map = vec![vec![vec![]; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        EntityStackMap {
            map: map,
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        }
    }

    pub fn clear(&mut self) {
        self.map = vec![vec![vec![]; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    }

    pub fn remove(&mut self, entity: &Entity, p: (i32, i32)) -> Option<Entity> {
        self.map[p.0 as usize][p.1 as usize].iter()
            .position(|e| e == entity)
            .map(|index| self.map[p.0 as usize][p.1 as usize].swap_remove(index))
    }

    pub fn is_empty(&self, p: (i32, i32)) -> bool {
        self.map[p.0 as usize][p.1 as usize].is_empty()
    }

    pub fn push(&mut self, entity: &Entity, p: (i32, i32)) {
        self.map[p.0 as usize][p.1 as usize].push(*entity);
    }

    pub fn pop(&mut self, p: (i32, i32)) -> Option<Entity> {
        self.map[p.0 as usize][p.1 as usize].pop()
    }

    pub fn get(&self, p: (i32, i32)) -> Iter<Entity> {
        self.map[p.0 as usize][p.1 as usize].iter()
    }
}
