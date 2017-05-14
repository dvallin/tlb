use specs::{ Entity };

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

pub struct EntityMap {
    map: Vec<Vec<Vec<Entity>>>,
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

    pub fn push(&mut self, entity: &Entity, x: i32, y: i32) {
        self.map[x as usize][y as usize].push(*entity);
    }

    pub fn pop(&mut self, x: i32, y: i32) -> Option<Entity> {
        self.map[x as usize][y as usize].pop()
    }
}
