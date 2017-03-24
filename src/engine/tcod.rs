use tcod::console::*;
use tcod::system::*;

use specs::{ World, Join };

use components::renderable::{ Renderable };
use tilemap::{ TileMap };

const LIMIT_FPS: i32 = 20;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

pub struct Tcod {
    root: Root,
    console: Offscreen,
}
impl Tcod {
    pub fn new() -> Tcod {
        let root = Root::initializer()
            .font("fonts/arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();
        set_fps(LIMIT_FPS);

        Tcod {
            root: root,
            console: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        }
    }

    pub fn render(&mut self, world: &mut World) {
        let entities = world.entities();
        let renderables = world.read::<Renderable>();
        let tilemap = world.read_resource::<TileMap>();

        for (renderable, _entity) in (&renderables, &entities).iter() {
            renderable.draw(&mut self.console);
        }
        tilemap.draw(&mut self.console);

        blit(&mut self.console, (0, 0), (MAP_WIDTH, MAP_HEIGHT),
             &mut self.root,(0, 0), 1.0, 1.0);
        self.root.flush();

        for (renderable, _entity) in (&renderables, &entities).iter() {
            renderable.clear(&mut self.console);
        }
    }
}
