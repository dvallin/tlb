use specs::{ Component, HashMapStorage, VecStorage, Entity };
use components::appearance::{ Renderable };
use components::common::{ Description };
use tcod::colors::{ self };

pub enum Type {
    Item,
    Consumable,
    Equipment,
    Weapon,
}

pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
    Epic,
}

pub enum Item {
    DartGun,
    FlickKnife,
    HitachiRam,
    Lighter,
    Manriki,
    PocketVtr,
    Shuriken,
    Simstim,
    Watch,
}

impl Component for Item {
    type Storage = VecStorage<Item>;
}

pub fn get_renderable(item: &Item) -> Renderable {
    let (t, r) = get_type(item);
    Renderable::new(
        match t {
            Type::Item => 'i',
            Type::Consumable => 'c',
            Type::Equipment => 'e',
            Type::Weapon => 'w',
        },
        match r {
            Rarity::Common => colors::GREY,
            Rarity::Uncommon => colors::GREEN,
            Rarity::Rare => colors::BLUE,
            Rarity::Unique => colors::PURPLE,
            Rarity::Epic => colors::GOLD,
        })
}

pub fn get_type(item: &Item) -> (Type, Rarity) {
    match *item {
        Item::Lighter | Item::Watch
            => (Type::Item, Rarity::Common),
        Item::HitachiRam
            => (Type::Item, Rarity::Uncommon),

        Item::PocketVtr
            => (Type::Equipment, Rarity::Uncommon),
        Item::Simstim
            => (Type::Equipment, Rarity::Rare),

        Item::FlickKnife | Item::Shuriken
            => (Type::Weapon, Rarity::Common),
        Item::Manriki
            => (Type::Weapon, Rarity::Uncommon),
        Item::DartGun
            => (Type::Weapon, Rarity::Rare),
    }
}

pub fn get_description(item: &Item) -> Description {
    match *item {
        Item::DartGun => Description::new("Dart gun", "Chinese dan-inject dart gun with a label stating \"Property of Jiuzhaigou Horse Conservative\""),
        Item::FlickKnife => Description::new("Flick knife", ""),
        Item::HitachiRam => Description::new("Hitachi HR 5MB RAM", "Adds 15 million characters of high speed random access memory. Only compatible with the Hitachi Z-80 main frame."),
        Item::Lighter => Description::new("Lighter", "A cerosine fueled lighter"),
        Item::Manriki => Description::new("Weighted manriki chains", "The thousand power chain of ancient japanese, made from old german steel."),
        Item::PocketVtr => Description::new("Pocket VTR", "Handheld video tape recording device"),
        Item::Shuriken => Description::new("Shuriken", "A traditional japanese conceiled weapon"),
        Item::Simstim => Description::new("Simstim deck", "Remotly simulates stimuli captured from another person to the wearer"),
        Item::Watch => Description::new("Watch", "A plastic watch"),
    }
}

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

pub struct ItemMap {
    map: Vec<Vec<Vec<Entity>>>,
    width: i32,
    height: i32,
}

impl ItemMap {
    pub fn new() -> Self {
        let map = vec![vec![vec![]; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        ItemMap {
            map: map,
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        }
    }

    pub fn push(&mut self, entity: &Entity, x: i32, y: i32) {
        self.map[x as usize][y as usize].push(*entity);
    }

    pub fn pop(&mut self, x: i32, y: i32) -> Option<Entity> {
        self.map[x as usize][y as usize].pop()
    }
}
