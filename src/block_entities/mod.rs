use banners::Banners;
use fastnbt::Value;
use serde::{Deserialize, Serialize};

pub mod banners;
pub mod barrel;
pub mod beacon;
pub mod beehive;
pub mod blast_furnace;
pub mod brewing_stand;
pub mod calibrated_sculk_sensor;
pub mod campfire;
pub mod chest;
pub mod chiseled_bookshelf;
pub mod command_block;
pub mod comparator;
pub mod conduit;
pub mod crafter;
pub mod decorated_pot;
pub mod dispenser;
pub mod dropper;
pub mod enchanting_table;
pub mod end_gateway;
pub mod furnace;
pub mod hanging_sign;
pub mod hopper;
pub mod jigsaw;
pub mod jukebox;
pub mod lectern;
pub mod mob_spawner;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum BlockEntityData<'a> {
    #[serde(borrow)]
    Banners(banners::Banners<'a>),

    #[serde(borrow)]
    Barrel(barrel::Barrel<'a>),

    #[serde(borrow)]
    Beacon(beacon::Beacon<'a>),

    Bed,

    Beehive(beehive::Beehive),

    Bell,

    #[serde(borrow)]
    BlastFurnace(blast_furnace::BlastFurnace<'a>),

    #[serde(borrow)]
    BrewingStand(brewing_stand::BrewingStand<'a>),

    #[serde(borrow)]
    CalibratedSculkSensor(calibrated_sculk_sensor::CalibratedSculkSensor<'a>),

    #[serde(borrow)]
    Campfire(campfire::Campfire<'a>),

    #[serde(borrow)]
    ChiseledBookshelf(chiseled_bookshelf::ChiseledBookshelf<'a>),

    #[serde(borrow)]
    Chest(chest::Chest<'a>),

    Comparator(comparator::Comparator),

    #[serde(borrow)]
    CommandBlock(command_block::CommandBlock<'a>),

    Conduit(conduit::Conduit),

    #[serde(borrow)]
    Crafter(crafter::Crafter<'a>),

    DaylightDetector,

    #[serde(borrow)]
    DecoratedPot(decorated_pot::DecoratedPot<'a>),

    #[serde(borrow)]
    Dispenser(dispenser::Dispenser<'a>),

    #[serde(borrow)]
    Dropper(dropper::Dropper<'a>),

    #[serde(borrow)]
    EnchantingTable(enchanting_table::EnchantingTable<'a>),

    EnderChest,

    EndGateway(end_gateway::EndGateway),

    EndPortal,

    #[serde(borrow)]
    Furnace(furnace::Furnace<'a>),

    #[serde(borrow)]
    Hopper(hopper::Hopper<'a>),

    #[serde(borrow)]
    Jigsaw(jigsaw::Jigsaw<'a>),

    #[serde(borrow)]
    Jukebox(jukebox::Jukebox<'a>),

    #[serde(borrow)]
    Lectern(lectern::Lectern<'a>),

    #[serde(borrow)]
    MobSpawner(mob_spawner::MobSpawner<'a>),

    Other(Option<Value>),
    None,
}

/// Converts a `BlockEntityData` enum to a `HashMap<String, Value>`.
macro_rules! to_value_map {
    ($value:expr) => {
        match fastnbt::to_value($value) {
            Ok(value) => Some(value),
            _ => None,
        }
    };
}

// macro_rules! from_value_map {
//     ($lifetime:tt, $data:expr, $type:tt) => {{
//         let map = Value::Compound($data);

//         let block_entity: $type = match fastnbt::from_value(&$lifetime map) {
//             Ok(block_entity) => block_entity,
//             Err(_) => return BlockEntityData::Other(Some(map.clone())),
//         };

//         block_entity
//     }};
// }

// fn from_value_map<'a, T>(data: HashMap<String, Value>) -> T
// where
//     T: Deserialize<'a>,
// {
//     let map = Value::Compound(data);

//     match fastnbt::from_value(&map) {
//         Ok(block_entity) => block_entity,
//         Err(_) => panic!("Failed to deserialize block entity data."),
//     }
// }

impl<'a> BlockEntityData<'a> {
    pub fn new(id: &str, data: &'a Value) -> BlockEntityData<'a> {
        match id {
            "minecraft:white_banner"
            | "minecraft:orange_banner"
            | "minecraft:magenta_banner"
            | "minecraft:light_blue_banner"
            | "minecraft:yellow_banner"
            | "minecraft:lime_banner"
            | "minecraft:pink_banner"
            | "minecraft:gray_banner"
            | "minecraft:light_gray_banner"
            | "minecraft:cyan_banner"
            | "minecraft:purple_banner"
            | "minecraft:blue_banner"
            | "minecraft:brown_banner"
            | "minecraft:green_banner"
            | "minecraft:red_banner"
            | "minecraft:black_banner"
            | "minecraft:white_wall_banner"
            | "minecraft:orange_wall_banner"
            | "minecraft:magenta_wall_banner"
            | "minecraft:light_blue_wall_banner"
            | "minecraft:yellow_wall_banner"
            | "minecraft:lime_wall_banner"
            | "minecraft:pink_wall_banner"
            | "minecraft:gray_wall_banner"
            | "minecraft:light_gray_wall_banner"
            | "minecraft:cyan_wall_banner"
            | "minecraft:purle_wall_banner"
            | "minecraft_blue_wall_banner"
            | "minecraft:brown_wall_banner"
            | "minecraft_green_wall_banner"
            | "minecraft:red_wall_banner"
            | "minecraft:black_wall_banner" => {
                let banners: Banners = match fastnbt::from_value(data) {
                    Ok(banners) => banners,
                    Err(_) => return BlockEntityData::Other(Some(data.clone())),
                };

                BlockEntityData::Banners(banners)
            }
            // "minecraft:end_gateway" => {
            //     let value = from_value_map(data);
            //     BlockEntityData::EndGateway(value)
            // }
            // TODO: Implement the rest of the block entities.
            _ => BlockEntityData::Other(Some(data.clone())),
        }
    }

    pub fn as_value(&'a self) -> Option<Value> {
        match self {
            BlockEntityData::Banners(banners) => to_value_map!(banners),
            BlockEntityData::Barrel(barrel) => to_value_map!(barrel),
            BlockEntityData::Beacon(beacon) => to_value_map!(beacon),
            BlockEntityData::Bed => None,
            BlockEntityData::Beehive(beehive) => to_value_map!(beehive),
            BlockEntityData::Bell => None,
            BlockEntityData::BlastFurnace(blast_furnace) => to_value_map!(blast_furnace),
            BlockEntityData::BrewingStand(brewing_stand) => to_value_map!(brewing_stand),
            BlockEntityData::CalibratedSculkSensor(calibrated_sculk_sensor) => {
                to_value_map!(calibrated_sculk_sensor)
            }
            BlockEntityData::Campfire(campfire) => to_value_map!(campfire),
            BlockEntityData::ChiseledBookshelf(chiseled_bookshelf) => {
                to_value_map!(chiseled_bookshelf)
            }
            BlockEntityData::Chest(chest) => to_value_map!(chest),
            BlockEntityData::Comparator(comparator) => to_value_map!(comparator),
            BlockEntityData::CommandBlock(command_block) => to_value_map!(command_block),
            BlockEntityData::Conduit(conduit) => to_value_map!(conduit),
            BlockEntityData::Crafter(crafter) => to_value_map!(crafter),
            BlockEntityData::DaylightDetector => None,
            BlockEntityData::DecoratedPot(decorated_pot) => to_value_map!(decorated_pot),
            BlockEntityData::Dispenser(dispenser) => to_value_map!(dispenser),
            BlockEntityData::Dropper(dropper) => to_value_map!(dropper),
            BlockEntityData::EnchantingTable(enchanting_table) => to_value_map!(enchanting_table),
            BlockEntityData::EnderChest => None,
            BlockEntityData::EndGateway(end_gateway) => to_value_map!(end_gateway),
            BlockEntityData::EndPortal => None,
            BlockEntityData::Furnace(furnace) => to_value_map!(furnace),
            BlockEntityData::Hopper(hopper) => to_value_map!(hopper),
            BlockEntityData::Jigsaw(jigsaw) => to_value_map!(jigsaw),
            BlockEntityData::Jukebox(jukebox) => to_value_map!(jukebox),
            BlockEntityData::Lectern(lectern) => to_value_map!(lectern),
            BlockEntityData::MobSpawner(mob_spawner) => to_value_map!(mob_spawner),

            BlockEntityData::Other(value) => value.clone(),
            BlockEntityData::None => None,
        }
    }
}
