use tcod::system::*;
use tcod::console::*;

use specs::{ World, Join };

use components::appearance::{ Renderable };
use components::space::{ Position };
use tilemap::{ TileMap };

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;
const MAX_FPS: i32 = 60;

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

        set_fps(MAX_FPS);

        Tcod {
            root: root,
            console: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        }
    }

    pub fn render(&mut self, world: &mut World) {
        let entities = world.entities();
        let renderables = world.read::<Renderable>();
        let positions = world.read::<Position>();
        let tilemap = world.read_resource::<TileMap>();

        self.root.clear();
        tilemap.draw(&mut self.console);

        for (renderable, _entity, position) in (&renderables, &entities, &positions).iter() {
            self.console.set_default_foreground(renderable.color);
            self.console.put_char(position.x as i32, position.y as i32,
                                  renderable.character, BackgroundFlag::None);
        }
        blit(&mut self.console, (0, 0), (MAP_WIDTH, MAP_HEIGHT),
             &mut self.root,(0, 0), 1.0, 1.0);
        self.root.flush();
    }
}
