use tcod::system::*;
use tcod::console::*;
use tcod::chars::{ self };
use tcod::map::{Map as FovMap, FovAlgorithm};
use tcod::colors::{ Color };

use specs::{ World };

use tilemap::{ TileMap };
use geometry::{ Rect };

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;
const MAP_Y: i32 = SCREEN_HEIGHT - MAP_HEIGHT;

const PANEL_WIDTH: i32 = SCREEN_WIDTH;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = 0;

const MAX_FPS: i32 = 60;

const FOV_LIGHT_WALLS: bool = true;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

pub struct Tcod {
    root: Root,
    console: Offscreen,
    panel: Offscreen,
    fov: Vec<FovMap>,
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
            panel: Offscreen::new(PANEL_WIDTH, PANEL_HEIGHT),
            fov: vec![],
        }
    }

    pub fn create_fov(&mut self) -> usize {
        self.fov.push(FovMap::new(MAP_WIDTH, MAP_HEIGHT));
        self.fov.len() - 1
    }

    pub fn initialize_fov(&mut self, index: usize, world: &mut World) {
        let tilemap = world.read_resource::<TileMap>();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let see_through = !tilemap.is_sight_blocking(x, y);
                let walk_through = !tilemap.is_blocking(x, y);
                self.fov[index].set(x, y, see_through, walk_through);
            }
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.console.set_default_background(color);
        self.console.clear();
        self.panel.set_default_background(color);
        self.panel.clear();
    }

    pub fn switch_fullscreen(&mut self) {
        let fullscreen = self.root.is_fullscreen();
        println!("Switching fullscreen {}", !fullscreen);
        self.root.set_fullscreen(!fullscreen);
    }

    pub fn flush(&mut self) {
        self.root.clear();
        blit(&mut self.console, (0, 0), (MAP_WIDTH, MAP_HEIGHT),
             &mut self.root,(0, MAP_Y), 1.0, 1.0);
        blit(&mut self.panel, (0, 0), (PANEL_WIDTH, PANEL_HEIGHT),
             &mut self.root,(0, PANEL_Y), 1.0, 1.0);
        self.root.flush();
    }

    pub fn is_in_fov(&self, x: i32, y: i32) -> bool {
        self.fov.iter()
            .any(|f| f.is_in_fov(x,y))
    }

    pub fn render(&mut self, x: i32, y: i32, bgcolor: Color, fgcolor: Color, character: char) {
        self.console.set_default_foreground(fgcolor);
        self.console.set_char_background(x, y, bgcolor, BackgroundFlag::Set);
        self.console.put_char(x, y, character, BackgroundFlag::None);
    }

    pub fn render_character(&mut self, x: i32, y: i32, fgcolor: Color, character: char) {
        self.console.set_default_foreground(fgcolor);
        self.console.put_char(x, y, character, BackgroundFlag::None);
    }

    pub fn render_text(&mut self, x: i32, y: i32, bgcolor: Color, fgcolor: Color, text: &String) {
        self.panel.set_default_foreground(fgcolor);
        self.panel.set_default_background(bgcolor);
        self.panel.print_ex(x, y, BackgroundFlag::Set, TextAlignment::Left, text);
    }

    pub fn render_box(&mut self, rect: &Rect, bgcolor: Color, fgcolor: Color) {
        self.panel.set_default_foreground(fgcolor);
        self.panel.set_default_background(bgcolor);
        self.panel.put_char(rect.left(), rect.top(), chars::NW, BackgroundFlag::None);
        self.panel.put_char(rect.right(), rect.top(), chars::NE, BackgroundFlag::None);
        self.panel.put_char(rect.left(), rect.bottom(), chars::SW, BackgroundFlag::None);
        self.panel.put_char(rect.right(), rect.bottom(), chars::SE, BackgroundFlag::None);
        for i in rect.left() + 1 .. rect.right() {
            self.panel.put_char(i, rect.top(), chars::HLINE, BackgroundFlag::None);
            self.panel.put_char(i, rect.bottom(), chars::HLINE, BackgroundFlag::None);
        }
        for i in rect.top() + 1 .. rect.bottom() {
            self.panel.put_char(rect.left(), i, chars::VLINE, BackgroundFlag::None);
            self.panel.put_char(rect.right(), i, chars::VLINE, BackgroundFlag::None);
        }
    }

    pub fn compute_fov(&mut self, index: usize, x: i32, y: i32, radius: i32) {
        self.fov[index].compute_fov(x, y, radius, FOV_LIGHT_WALLS, FOV_ALGO);
    }
}
