use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MobSpawner<'a> {
    /// Ticks until next spawn. If 0, it spawns immediately when a player enters its range. If set to -1 (this state never occurs in a natural spawner; it seems to be a feature accessed only via NBT editing), the spawner resets this and SpawnData as though it had just completed a successful spawn cycle, immediately when a player enters its range. Setting this to -1 can be useful if the player wants the game to properly randomize the spawner's Delay and SpawnData, rather than starting with pre-defined values.
    #[serde(rename = "Delay")]
    pub delay: i16,

    /// Overrides the maximum number of nearby (within a box of SpawnRange*2+1 × SpawnRange*2+1 × 8 centered around the spawner block) entities whose IDs match this spawner's entity ID. This is relative to a mob's hitbox, not its physical position. Also, all entities within all chunk sections (16×16×16 cubes) overlapped by this box are tested for their ID and hitbox overlap, rather than just entities within the box, meaning that a large amount of entities outside the box (or within it, of course) can cause substantial lag
    #[serde(rename = "MaxNearbyEntities")]
    pub max_nearby_entities: i16,

    /// The maximum random delay for the next spawn delay. Requires the MinSpawnDelay and SpawnCount properties to also be set.
    #[serde(rename = "MaxSpawnDelay")]
    pub max_spawn_delay: i16,

    /// The minimum random delay for the next spawn delay. May be equal to MaxSpawnDelay. Requires the SpawnCount property to also be set, otherwise it defaults to 0.
    #[serde(rename = "MinSpawnDelay")]
    pub min_spawn_delay: i16,

    /// Overrides the block radius of the sphere of activation by players for this spawner. For every gametick, a spawner checks all players in the current world to test whether a player is within this sphere. Requires the MaxNearbyEntities property to also be set.
    #[serde(rename = "RequiredPlayerRange")]
    pub required_player_range: i16,

    /// How many mobs to attempt to spawn each time. Requires the MinSpawnDelay property to also be set.
    #[serde(rename = "SpawnCount")]
    pub spawn_count: i16,

    /// Contains tags to copy to the next spawned entity(s) after spawning. Any of the entity or mob tags may be used. If a spawner specifies any of these tags, almost all variable data such as mob equipment, villager profession, sheep wool color, etc., are not automatically generated, and must also be manually specified (that this does not apply to position data, which are randomized as normal unless Pos is specified. Similarly, unless Size and Health are specified for a Slime or Magma Cube, these are still randomized). This also determines the appearance of the miniature entity spinning in the spawner cage. Warning: If SpawnPotentials exists, this tag gets overwritten after the next spawning attempt: see above for more details.
    #[serde(borrow)]
    #[serde(rename = "SpawnData")]
    pub spawn_data: SpawnData<'a>,

    /// Optional. List of possible entities to spawn. If this tag does not exist, but SpawnData exists, Minecraft generates it the next time the spawner tries to spawn an entity. The generated list contains a single entry derived from the SpawnData tag.
    #[serde(borrow)]
    #[serde(rename = "SpawnPotentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_potentials: Option<Vec<PotentialSpawn<'a>>>,

    /// The radius around which the spawner attempts to place mobs randomly. The spawn area is square, includes the block the spawner is in, and is centered around the spawner's x,z coordinates - not the spawner itself. It is 2 blocks high, centered around the spawner's y coordinate (its bottom), allowing mobs to spawn as high as its top surface and as low as 1 block below its bottom surface. Vertical spawn coordinates are integers, while horizontal coordinates are floating point and weighted toward values near the spawner itself. Default value is 4.
    #[serde(rename = "SpawnRange")]
    pub spawn_range: i16,
}

/// A potential future spawn. After the spawner makes an attempt at spawning, it chooses one of these entries at random and uses it to prepare for the next spawn, overwriting SpawnData.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PotentialSpawn<'a> {
    /// The chance that this spawn gets picked in comparison to other spawn weights. Must be positive and at least 1.
    pub weight: i32,

    #[serde(borrow)]
    pub data: SpawnData<'a>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SpawnData<'a> {
    /// An entity, including the entity id.
    #[serde(borrow)]
    pub entity: Entity<'a>,

    /// Optional custom fields to override spawning requirements.
    pub custom_spawn_rules: SpawnRules,

    /// Optional. Determines the equipment the entity will wear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment: Option<Equipment<'a>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SpawnRules {
    /// Can either be a single value, or a compound containing min_inclusive and max_inclusive. Overrides the block light check when spawning the entity.
    pub block_light_limit: i32,

    /// Can either be a single value, or a compound containing min_inclusive and max_inclusive. Overrides the sky light check when spawning the entity.
    pub sky_light_limit: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Equipment<'a> {
    /// Resource location of a loot table to use to generate the equipment
    pub loot_table: Cow<'a, str>,

    /// Optional. When a determines the drop chances for every slot. When a , controls the drop chances per slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_drop_chances: Option<DropChanceType>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum DropChanceType {
    All(f32),
    Indiviual(DropChances),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DropChances {
    /// Optional. Drop chance of the boots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feet: Option<f32>,

    /// Optional. Drop chance of the leggings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<f32>,

    /// Optional. Drop chance of the chestplate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chest: Option<f32>,

    /// Optional. Drop chance of the helmet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<f32>,

    /// Optional. Drop chance of the body armor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<f32>,

    /// Optional. Drop chance of the weapon in the main hand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainhand: Option<f32>,

    /// Optional. Drop chance of the weapon in the off hand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offhand: Option<f32>,
}

#[cfg(test)]
#[test]
fn basic_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Delay": 20i16,
        "MaxNearbyEntities": 6i16,
        "MaxSpawnDelay": 800i16,
        "MinSpawnDelay": 200i16,
        "RequiredPlayerRange": 16i16,
        "SpawnCount": 4i16,
        "SpawnData": {
            "entity": {
                "Air": 32i16,
                "CustomName": "Mr Zombie",
                "FallDistance": 0.0f32,
                "Glowing": true,
                "HasVisualFire": true,
                "id": "minecraft:zombie",
                "Invulnerable": false,
                "Motion": [0.0f64, 0.0f64, 0.0f64],
                "NoGravity": false,
                "OnGround": true,
                "PortalCooldown": 0,
                "Pos": [5.5f64, 6.3f64, 2.1f64],
                "Rotation": [0.0f32, 0.0f32],
                "UUID": [I; 1, 2, 3, 4,]
            },
            "custom_spawn_rules": {
                "block_light_limit": 10,
                "sky_light_limit": 15
            },
            "equipment": {
                "loot_table": "minecraft:entities/zombie",
                "slot_drop_chances": {
                    "feet": 0.045,
                    "chest": 0.1,
                    "mainhand": 0.085
                }
            }
        },
        "SpawnPotentials": [
            {
                "weight": 1i32,
                "data": {
                    "entity": {
                        "Air": 32i16,
                        "CustomName": "Mr Zombie",
                        "FallDistance": 0.0f32,
                        "Glowing": true,
                        "HasVisualFire": true,
                        "id": "minecraft:zombie",
                        "Invulnerable": false,
                        "Motion": [0.0f64, 0.0f64, 0.0f64],
                        "NoGravity": false,
                        "OnGround": true,
                        "PortalCooldown": 0,
                        "Pos": [5.5f64, 6.3f64, 2.1f64],
                        "Rotation": [0.0f32, 0.0f32],
                        "UUID": [I; 1, 2, 3, 4,]
                    },
                    "custom_spawn_rules": {
                        "block_light_limit": 10,
                        "sky_light_limit": 15
                    },
                    "equipment": {
                        "loot_table": "minecraft:entities/zombie",
                        "slot_drop_chances": {
                            "feet": 0.045,
                            "chest": 0.1,
                            "mainhand": 0.085
                        }
                    }
                }
            }
        ],
        "SpawnRange": 4i16
    });

    let mob_spawner: MobSpawner = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(mob_spawner.delay, 20);
    assert_eq!(mob_spawner.max_nearby_entities, 6);
    assert_eq!(mob_spawner.max_spawn_delay, 800);
    assert_eq!(mob_spawner.min_spawn_delay, 200);
    assert_eq!(mob_spawner.required_player_range, 16);
    assert_eq!(mob_spawner.spawn_count, 4);
    assert_eq!(mob_spawner.spawn_data.entity.id, "minecraft:zombie");
    assert_eq!(
        mob_spawner.spawn_data.custom_spawn_rules.block_light_limit,
        10
    );
    assert_eq!(
        mob_spawner.spawn_data.custom_spawn_rules.sky_light_limit,
        15
    );
    assert_eq!(
        mob_spawner
            .spawn_data
            .equipment
            .as_ref()
            .unwrap()
            .loot_table,
        "minecraft:entities/zombie"
    );
    if let DropChanceType::Indiviual(chances) = mob_spawner
        .spawn_data
        .equipment
        .as_ref()
        .unwrap()
        .slot_drop_chances
        .as_ref()
        .unwrap()
    {
        assert_eq!(chances.chest, Some(0.1));
    } else {
        panic!("Expected individual drop chances");
    }
    assert_eq!(mob_spawner.spawn_potentials.as_ref().unwrap().len(), 1);
    assert_eq!(mob_spawner.spawn_potentials.as_ref().unwrap()[0].weight, 1);
    assert_eq!(
        mob_spawner.spawn_potentials.as_ref().unwrap()[0]
            .data
            .entity
            .id,
        "minecraft:zombie"
    );
    assert_eq!(
        mob_spawner.spawn_potentials.as_ref().unwrap()[0]
            .data
            .custom_spawn_rules
            .block_light_limit,
        10
    );
    assert_eq!(
        mob_spawner.spawn_potentials.as_ref().unwrap()[0]
            .data
            .custom_spawn_rules
            .sky_light_limit,
        15
    );
    assert_eq!(
        mob_spawner.spawn_potentials.as_ref().unwrap()[0]
            .data
            .equipment
            .as_ref()
            .unwrap()
            .loot_table,
        "minecraft:entities/zombie"
    );
    if let DropChanceType::Indiviual(chances) = mob_spawner.spawn_potentials.as_ref().unwrap()[0]
        .data
        .equipment
        .as_ref()
        .unwrap()
        .slot_drop_chances
        .as_ref()
        .unwrap()
    {
        assert_eq!(chances.feet, Some(0.045));
    } else {
        panic!("Expected individual drop chances");
    }
    assert_eq!(mob_spawner.spawn_range, 4);

    let nbt = fastnbt::to_value(&mob_spawner).unwrap();

    assert_eq!(nbt, nbt);
}
