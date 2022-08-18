//! This module contains utilities to convert strings into the proper Discord format.

use std::cmp::min;
use once_cell::sync::{Lazy};
use regex::{Regex};


pub trait Channelizable {
    /// Convert a string to an acceptable channel name by limiting it to 32 characters and by using  `kebab-lower-case`.
    fn channelify(&self) -> String;
}

impl Channelizable for String {
    fn channelify(&self) -> String {
        static REPLACE_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("[^a-z0-9]")
                .expect("Invalid REPLACE_PATTERN")
        });

        let s = self.as_str();
        let s = &s[..min(s.len(), 32)];
        let s = s.to_ascii_lowercase();
        let s = (*REPLACE_PATTERN).replace_all(&s, " ").into_owned();
        let s = s.trim();
        let s = s.replace(' ', "-");

        s
    }
}


