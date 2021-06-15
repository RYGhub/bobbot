use serenity::framework::standard::{Args};
use once_cell::sync::{Lazy};
use regex::{Regex};
use crate::basics::result::{BobResult, BobError};

/// Convert a string to an acceptable channel name using `kebab-lower-case`.
pub fn channelify(s: &str) -> String {
    static REPLACE_PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new("[^a-z0-9]")
            .expect("Invalid REPLACE_PATTERN")
    });

    let mut last = s.len();
    if last > 32 {
        last = 32;
    }

    let s = &s[..last];
    let s = s.to_ascii_lowercase();
    let s: String = (*REPLACE_PATTERN).replace_all(&s, " ").into_owned();
    let s = s.trim();
    let s = s.replace(" ", "-");

    s
}


/// Parse a single argument as a preset name.
pub fn parse_preset_name(args: &mut Args) -> BobResult<String> {
    args.single()
        .map_err(|_| BobError {msg: "Missing preset name argument"})
}


/// Parse the rest of the args as a channel name.
pub fn parse_channel_name(args: Args) -> BobResult<String> {
    let rest = args.rest();

    if rest.len() == 0 {
        return Err(BobError {msg: "Missing channel name argument"})
    }

    Ok(channelify(rest))
}
