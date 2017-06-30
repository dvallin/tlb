use specs::{ Component, HashMapStorage, VecStorage, Entity };
use components::appearance::{ Renderable };
use components::common::{ Description, ItemStats };
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
}

impl Component for Item {
    type Storage = VecStorage<Item>;
}

pub fn get_renderable(item: &Item) -> Renderable {
    use self::Type::*;
    use self::Rarity::*;
    let (r, t) = get_type(item);
    Renderable::new(
        match t {
            Item => 'i',
            Consumable => 'c',
            Equipment => 'e',
            Weapon => 'w',
        },
        match r {
            Common => colors::GREY,
            Uncommon => colors::GREEN,
            Rare => colors::BLUE,
            Unique => colors::PURPLE,
            Epic => colors::GOLD,
        })
}

pub fn get_stats(item: &Item) -> Option<ItemStats> {
    use self::ItemInstance::*;
    match item.instance {
        FlickKnife => Some (ItemStats { damage: 20.0, range: 1 }),
        Shuriken => Some (ItemStats { damage: 20.0, range: 5 }),
        Manriki => Some (ItemStats { damage: 40.0, range: 2 }),
        DartGun => Some (ItemStats { damage: 80.0, range: 10 }),
        _ => None,
    }
}

pub fn get_type(item: &Item) -> (Rarity, Type) {
    use self::ItemInstance::*;
    use self::Type::*;
    use self::Rarity::*;
    match item.instance {
        Lighter | Watch => (Common, Item),
        FlickKnife => (Common, Weapon),

        PocketVtr => (Uncommon, Equipment),
        Manriki | Shuriken => (Uncommon, Weapon),

        Simstim => (Rare, Equipment),
        DartGun => (Rare, Weapon),

        HitachiRam => (Epic, Item),
    }
}

pub fn get_description(item: &Item) -> Description {
    use self::ItemInstance::*;
    match item.instance {
        DartGun => Description::new("Dart gun", "Chinese dan-inject dart gun with a label stating \"Property of Jiuzhaigou Horse Conservative\""),
        FlickKnife => Description::new("Flick knife", ""),
        HitachiRam => Description::new("Hitachi HR 5MB RAM", "Adds 15 million characters of high speed random access memory. Only compatible with the Hitachi Z-80 main frame."),
        Lighter => Description::new("Lighter", "A cerosine fueled lighter"),
        Manriki => Description::new("Weighted manriki chains", "The thousand power chain of ancient japanese, made from old german steel."),
        PocketVtr => Description::new("Pocket VTR", "Handheld video tape recording device"),
        Shuriken => Description::new("Shuriken", "A traditional japanese conceiled weapon"),
        Simstim => Description::new("Simstim deck", "Remotly simulates stimuli captured from another person to the wearer"),
        Watch => Description::new("Watch", "A plastic watch"),
    }
}
