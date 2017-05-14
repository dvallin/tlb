use specs::{ Component, HashMapStorage, VecStorage, Entity };
use components::space::{ Position };
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

pub enum ItemInstance {
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

pub struct Item {
    pub instance: ItemInstance,
    pub spawn: Position,
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
    match item.instance {
        ItemInstance::Lighter | ItemInstance::Watch
            => (Type::Item, Rarity::Common),
        ItemInstance::HitachiRam
            => (Type::Item, Rarity::Epic),

        ItemInstance::PocketVtr
            => (Type::Equipment, Rarity::Uncommon),
        ItemInstance::Simstim
            => (Type::Equipment, Rarity::Rare),

        ItemInstance::FlickKnife | ItemInstance::Shuriken
            => (Type::Weapon, Rarity::Common),
        ItemInstance::Manriki
            => (Type::Weapon, Rarity::Uncommon),
        ItemInstance::DartGun
            => (Type::Weapon, Rarity::Rare),
    }
}

pub fn get_description(item: &Item) -> Description {
    match item.instance {
        ItemInstance::DartGun => Description::new("Dart gun", "Chinese dan-inject dart gun with a label stating \"Property of Jiuzhaigou Horse Conservative\""),
        ItemInstance::FlickKnife => Description::new("Flick knife", ""),
        ItemInstance::HitachiRam => Description::new("Hitachi HR 5MB RAM", "Adds 15 million characters of high speed random access memory. Only compatible with the Hitachi Z-80 main frame."),
        ItemInstance::Lighter => Description::new("Lighter", "A cerosine fueled lighter"),
        ItemInstance::Manriki => Description::new("Weighted manriki chains", "The thousand power chain of ancient japanese, made from old german steel."),
        ItemInstance::PocketVtr => Description::new("Pocket VTR", "Handheld video tape recording device"),
        ItemInstance::Shuriken => Description::new("Shuriken", "A traditional japanese conceiled weapon"),
        ItemInstance::Simstim => Description::new("Simstim deck", "Remotly simulates stimuli captured from another person to the wearer"),
        ItemInstance::Watch => Description::new("Watch", "A plastic watch"),
    }
}
