use serenity::framework::standard::{Args};
use once_cell::sync::{Lazy};
use regex::{Regex};
use crate::errors::{BobResult, user_error};
use std::cmp::min;

/// Convert a string to an acceptable channel name by limiting it to 32 characters and by using  `kebab-lower-case`.
pub fn channelify(s: &str) -> String {
    static REPLACE_PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new("[^a-z0-9]")
            .expect("Invalid REPLACE_PATTERN")
    });

    let s = &s[..min(s.len(), 32)];
    let s = s.to_ascii_lowercase();
    let s: String = (*REPLACE_PATTERN).replace_all(&s, " ").into_owned();
    let s = s.trim();
    let s = s.replace(" ", "-");

    s
}


/// Parse a single argument as a preset name.
pub fn parse_preset_name(args: &mut Args) -> BobResult<String> {
    args.single()
        .map_err(|_| user_error("Missing preset name."))
}


/// Parse the rest of the args as a channel name.
pub fn parse_channel_name(args: Args) -> BobResult<String> {
    let rest = args.rest();

    match rest.len() {
        0 => Err(user_error("Missing channel name.")),
        _ => Ok(channelify(rest))
    }
}
