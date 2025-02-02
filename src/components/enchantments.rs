//! Enchantments on an item.

use crate::{kv::KVPair, traits::FromCompoundNbt};

/// Enchantments on an item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enchantments {
    /// Contains key-value pairs of levels of enchantments on this item that affect the way the item works.  
    pub levels: KVPair<i32>,

    /// Show or hide enchantments on this item's tooltip. Defaults to true.
    #[cfg_attr(feature = "serde", serde(default = "crate::util::default_true"))]
    pub show_in_tooltip: bool,
}

impl FromCompoundNbt for Enchantments {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(levels) = nbt.compound("levels") {
            // Field compound

            let levels = KVPair::<i32>::from_compound_nbt(&levels)?;
            let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

            Ok(Enchantments {
                levels,
                show_in_tooltip,
            })
        } else {
            // key value only
            let levels = KVPair::<i32>::from_compound_nbt(nbt)?;

            // this could be an enum but eh, show_in_tooltip is default true anyway
            Ok(Enchantments {
                levels,
                show_in_tooltip: true,
            })
        }
    }
}
