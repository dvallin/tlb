use tcod::system::*;
use tcod::console::*;
use tcod::map::{Map as FovMap, FovAlgorithm};
use tcod::colors::{ Color };

use specs::{ World, Join };

use components::appearance::{ Renderable };
use components::space::{ Position };
use tilemap::{ TileMap };

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;
const MAX_FPS: i32 = 60;

const FOV_LIGHT_WALLS: bool = true;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

pub struct Tcod {
    root: Root,
    console: Offscreen,
    fov: FovMap,
}
impl Tcod {
    pub fn new() -> Tcod {
        let root = Root::initializer()
            .font("fonts/arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();

        set_fps(MAX_FPS);

        Tcod {
            root: root,
            console: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
            fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        }
    }

    pub fn initialize(&mut self, world: &mut World) {
        let tilemap = world.read_resource::<TileMap>();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                self.fov.set(x, y,
                               !tilemap.is_sight_blocking(x, y),
                               !tilemap.is_blocking(x, y));
            }
        }
    }

    pub fn clear(&mut self) {
        self.console.clear();
    }

    pub fn flush(&mut self) {
        self.root.clear();
        blit(&mut self.console, (0, 0), (MAP_WIDTH, MAP_HEIGHT),
             &mut self.root,(0, 0), 1.0, 1.0);
        self.root.flush();
    }

    pub fn is_in_fov(&self, x: i32, y: i32) -> bool {
        self.fov.is_in_fov(x,y)
    }

    pub fn render(&mut self, x: i32, y: i32, bgcolor: Color, fgcolor: Color, character: char) {
        self.console.set_char_foreground(x, y, fgcolor);
        self.console.set_char_background(x, y, bgcolor, BackgroundFlag::Set);
        self.console.put_char(x, y, character, BackgroundFlag::None);
    }

    pub fn compute_fov(&mut self, x: i32, y: i32, radius: i32) {
        self.fov.compute_fov(x, y, radius, FOV_LIGHT_WALLS, FOV_ALGO);
    }
}
