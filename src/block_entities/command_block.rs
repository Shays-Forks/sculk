use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_optional_string, get_owned_string},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommandBlock {
    /// Allows to activate the command without the requirement of a redstone signal.
    pub auto: bool,

    /// The command to issue to the server.
    ///
    /// `Command`
    pub command: String,

    /// Indicates whether a conditional command block had its condition met when last activated. True if not a conditional command block.
    ///
    /// `conditonMet`
    pub condition_met: bool,

    /// Optional. The name JSON text component of this command block, replacing the usual '@' when using commands such as /say and /tell.
    ///
    /// `CustomName`
    pub custom_name: Option<String>,

    /// stores the tick a chain command block was last executed in.
    ///
    /// `LastExecution`
    pub last_execution: i64,

    /// The last line of output generated by the command block. Still stored even if the game rule commandBlockOutput is false. Appears in the GUI of the block when right-clicked, and includes a timestamp of when the output was produced.
    ///
    /// `LastOutput`
    pub last_output: String,

    /// States whether or not the command block is powered by redstone or not.
    pub powered: bool,

    /// Represents the strength of the analog signal output by redstone comparators attached to this command block.
    ///
    /// `SuccessCount`
    pub success_count: i32,

    /// Determines whether the LastOutput is stored. Can be toggled in the GUI by clicking a button near the "Previous Output" textbox. Caption on the button indicates current state: "O" if true, "X" if false.
    ///
    /// `TrackOutput`
    pub track_output: bool,

    /// Defaults to true. If set to false, loops can be created where the same command block can run multiple times in one tick.
    ///
    /// `UpdateLastExecution`
    pub update_last_execution: bool,
}

impl FromCompoundNbt for CommandBlock {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let auto = get_bool(&nbt, "auto");
        let command = get_owned_string(&nbt, "Command")?;
        let condition_met = get_bool(&nbt, "conditionMet");
        let custom_name = get_owned_optional_string(&nbt, "CustomName");
        let last_execution = nbt
            .long("LastExecution")
            .ok_or(SculkParseError::MissingField("LastExecution".into()))?;
        let last_output = get_owned_string(&nbt, "LastOutput")?;
        let powered = get_bool(&nbt, "powered");
        let success_count = nbt
            .int("SuccessCount")
            .ok_or(SculkParseError::MissingField("SuccessCount".into()))?;
        let track_output = get_bool(&nbt, "TrackOutput");
        let update_last_execution = get_bool(&nbt, "UpdateLastExecution");

        Ok(CommandBlock {
            auto,
            command,
            condition_met,
            custom_name,
            last_execution,
            last_output,
            powered,
            success_count,
            track_output,
            update_last_execution,
        })
    }
}
