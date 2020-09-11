pub mod create_temp_channel;

use once_cell::sync::Lazy;
use regex::Regex;
use serenity::model::prelude::PermissionOverwrite;


/// Convert a string to **kebab-lower-case**.
pub fn kebabify(s: &str) -> String {
    static REPLACE_PATTERN: Lazy<Regex> = Lazy::new(|| {Regex::new("[^a-z0-9]").unwrap()});

    let mut last = s.len();
    if last > 100 {
        last = 100;
    }

    let s = &s[..last];
    let s = s.to_ascii_lowercase();
    let s: String = (*REPLACE_PATTERN).replace_all(&s, " ").into_owned();
    let s = s.trim();
    let s = s.replace(" ", "-");

    s
}


/// A container for serializing PermissionOverwrites with a `[[permissions]]` header.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PermissionOverwritesContainer {
    pub permissions: Vec<PermissionOverwrite>
}
