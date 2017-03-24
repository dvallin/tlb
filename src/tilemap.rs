#[derive(Clone, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
    explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            explored: false,
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            explored: false,
            blocked: true,
            block_sight: true,
        }
    }
}

struct TileMap {
    map: Vec<Vec<Tile>>,
    fov: tcod::map::Map,

    console: tcod::console::Offscreen,
    width: i32,
    height: i32,
}

impl TileMap {
    fn create() -> Self {
        const MAP_WIDTH: i32 = 80;
        const MAP_HEIGHT: i32 = 43;
        let map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        TileMap {
            console: tcod::console::Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
            fov:  tcod::map::Map::new(MAP_WIDTH, MAP_HEIGHT),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            map: map,
        }
    }

    fn render(&mut self) {
        use tcod::Console;
        use tcod::colors::{ Color };
        const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
        for y in 0..self.height {
            for x in 0..self.width {
                let color = COLOR_DARK_WALL;
                self.console.set_char_background(x, y, color, tcod::console::BackgroundFlag::Set);
            }
        }
    }

    fn blit(&mut self, root: &mut tcod::console::Root) {
        tcod::console::blit(&mut self.console, (0, 0), (self.width, self.height),
                            root,(0, 0), 1.0, 1.0);
    }
}
