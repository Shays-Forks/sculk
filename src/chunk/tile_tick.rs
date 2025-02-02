use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_string};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileTick {
    /// The ID of the block; used to activate the correct block update procedure.
    pub i: String,

    /// If multiple tile ticks are scheduled for the same tick, tile ticks with lower p are processed first. If they also have the same p, the order is unknown.
    pub p: i32,

    /// The number of ticks until processing should occur. May be negative when processing is overdue.
    pub t: i32,

    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Z position
    pub z: i32,
}

impl FromCompoundNbt for TileTick {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let i = get_owned_string(nbt, "i")?;
        let p = nbt
            .int("p")
            .ok_or(SculkParseError::MissingField("p".into()))?;
        let t = nbt
            .int("t")
            .ok_or(SculkParseError::MissingField("t".into()))?;

        let x = nbt
            .int("x")
            .ok_or(SculkParseError::MissingField("x".into()))?;
        let y = nbt
            .int("y")
            .ok_or(SculkParseError::MissingField("y".into()))?;
        let z = nbt
            .int("z")
            .ok_or(SculkParseError::MissingField("z".into()))?;

        Ok(TileTick { i, p, t, x, y, z })
    }
}
