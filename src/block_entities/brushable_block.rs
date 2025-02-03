use crate::traits::FromCompoundNbt;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BrushableBlock {}

impl FromCompoundNbt for BrushableBlock {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        // TODO: Finish BrushableBlocks
        Ok(BrushableBlock {})
    }
}
