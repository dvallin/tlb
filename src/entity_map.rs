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

    pub fn remove(&mut self, _: &Entity, x: i32, y: i32) -> Option<Entity> {
        self.pop(x, y)
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.map[x as usize][y as usize].is_none()
    }

    pub fn push(&mut self, entity: &Entity, x: i32, y: i32) {
        self.map[x as usize][y as usize] = Some(*entity);
    }

    pub fn pop(&mut self, x: i32, y: i32) -> Option<Entity> {
        let e = self.map[x as usize][y as usize];
        self.map[x as usize][y as usize] = None;
        e
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        self.map[x as usize][y as usize]
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

    pub fn remove(&mut self, entity: &Entity, x: i32, y: i32) -> Option<Entity> {
        self.map[x as usize][y as usize].iter()
            .position(|e| e == entity)
            .map(|index| self.map[x as usize][y as usize].swap_remove(index))
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.map[x as usize][y as usize].is_empty()
    }

    pub fn push(&mut self, entity: &Entity, x: i32, y: i32) {
        self.map[x as usize][y as usize].push(*entity);
    }

    pub fn pop(&mut self, x: i32, y: i32) -> Option<Entity> {
        self.map[x as usize][y as usize].pop()
    }
}
