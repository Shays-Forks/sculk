use crate::{components::Components, error::SculkParseError, traits::FromCompoundNbt};
use simdnbt::borrow::{NbtCompound, NbtList};

/// The version of Minecraft that this library is designed to work with.  
/// Formatted exactly as minecraft versions are.
#[allow(dead_code)]
pub const MC_VERSION: &str = "1.21";

pub struct LootTableData {
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[cfg(feature = "serde")]
pub fn default_true() -> bool {
    true
}

pub fn get_loot_table_data(nbt: &NbtCompound) -> LootTableData {
    let loot_table = get_owned_optional_string(nbt, "LootTable");

    let loot_table_seed = nbt.long("LootTableSeed");

    LootTableData {
        loot_table,
        loot_table_seed,
    }
}

// TODO: convert get_owned_mutf8str to borrowed Cow
// Making this borrowed would save like 50µs per call.
// But i just dont know how to deal with it and its lifetimes.
pub fn get_owned_string(nbt: &NbtCompound, key: &'static str) -> Result<String, SculkParseError> {
    nbt
        .string(key)
        .map(|s| s.to_string())
        .ok_or(SculkParseError::InvalidField(key.into()))
}

pub fn get_owned_optional_string(nbt: &NbtCompound, key: &'static str) -> Option<String> {
    nbt.string(key).map(|s| s.to_string())
}

pub fn get_optional_lock<'a>(nbt: &NbtCompound) -> Option<String> {
    nbt.string("Lock").map(|s| s.to_string())
}

pub fn get_optional_name<'a>(nbt: &NbtCompound) -> Option<String> {
    nbt.string("CustomName").map(|s| s.to_string())
}

pub fn get_bool(nbt: &NbtCompound, key: &'static str) -> bool {
    nbt.byte(key).map(|b| b != 0).unwrap_or(false)
}

pub fn get_int_array(nbt: &NbtCompound, key: &'static str) -> Result<Vec<i32>, SculkParseError> {
    let list = match nbt.list(key) {
        Some(list) => list,
        None => return Ok(vec![]),
    };

    let mut arr = vec![];

    for item in list
        .ints()
        .ok_or(SculkParseError::InvalidField(key.into()))?
        .iter()
    {
        arr.push(*item);
    }

    Ok(arr)
}

pub fn get_doubles_array(
    nbt: &NbtCompound,
    key: &'static str,
) -> Result<Vec<f64>, SculkParseError> {
    let list = match nbt.list(key) {
        Some(list) => list,
        None => return Ok(vec![]),
    };

    let mut arr = vec![];

    for item in list
        .doubles()
        .ok_or(SculkParseError::InvalidField(key.into()))?
        .iter()
    {
        arr.push(*item);
    }

    Ok(arr)
}

pub fn get_t_compound_vec<T>(
    nbt: &NbtCompound,
    key: &'static str,
    nbt_conversion: fn(nbt: &NbtCompound) -> Result<T, SculkParseError>,
) -> Result<Vec<T>, SculkParseError> {
    let list = match nbt.list(key) {
        Some(list) => list,
        None => return Ok(vec![]),
    };

    if list.empty() {
        return Ok(vec![]);
    }

    let mut vec = vec![];

    for item in list
        .compounds()
        .ok_or(SculkParseError::InvalidField(key.into()))?
    {
        vec.push(nbt_conversion(&item)?);
    }

    Ok(vec)
}

pub fn get_t_list<T>(
    nbt: &NbtList,
    key: &'static str,
    nbt_conversion: fn(nbt: &NbtCompound) -> Result<T, SculkParseError>,
) -> Result<Vec<T>, SculkParseError> {
    Ok(if let Some(compounds) = nbt.compounds() {
        compounds
            .into_iter()
            .map(|nbt| nbt_conversion(&nbt))
            .collect::<Result<Vec<T>, SculkParseError>>()?
    } else {
        vec![]
    })
}

pub fn get_optional_components(nbt: &NbtCompound) -> Result<Option<Components>, SculkParseError> {
    match Components::from_compound_nbt(nbt) {
        Ok(components) => Ok(Some(components)),
        // Only return None if the field is missing
        Err(SculkParseError::MissingField(_)) => Ok(None),
        // Return the error if it's anything else
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
pub fn dump_nbt(nbt: &NbtCompound) {
    for (key, value) in nbt.iter() {
        println!("{:?}: {:?}", key, value.to_owned());
    }

    println!();
}
