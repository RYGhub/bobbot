//! This module contains utilities to easily parse Bob-specific command arguments.

use serenity::framework::standard::{Args};
use crate::utils::channel_names::{channelify};
use crate::errors::*;


/// This trait extends [Args] with additional methods to allow the parsing of some Bob-specific objects.
pub trait BobArgs {
    /// Parse a single argument as a preset name.
    fn preset_name(&mut self) -> BobResult<String>;

    /// Parse the rest of the args as a channel name, using [channelify].
    fn channel_name(self) -> BobResult<String>;
}

impl BobArgs for Args {
    fn preset_name(&mut self) -> BobResult<String> {
        self.single()
            .bob_catch(ErrorKind::User, "Missing preset name.")
    }

    fn channel_name(self) -> BobResult<String> {
        let rest = self.rest();

        match rest.len() {
            0 => Err(BobError::from_msg(ErrorKind::User, "Missing channel name.")),
            _ => Ok(channelify(rest))
        }
    }
}
