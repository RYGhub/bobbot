use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::error::{Error};
use crate::utils::discord_display::{DiscordDisplay};


pub enum ErrorKind {
    User,
    Admin,
    Developer,
    External,
}

#[derive(Debug, Clone)]
pub struct BobError {
    knd: ErrorKind,
    msg: Option<String>,
    err: Option<Box<dyn Error>>,
}

impl Display for BobError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.msg.is_some() && self.err.is_some() {
            write!(f, "{:?}: {}", &self.err.unwrap(), &self.msg.unwrap())
        }
        else if self.msg.is_some() {
            write!(f, "Error: {}", &self.msg.unwrap())
        }
        else if self.err.is_some() {
            write!(f, "{:?}: _no message_", &self.err.unwrap())
        }
        else {
            write!(f, "Error: _no message_")
        }
    }
}

impl Error for BobError {}

impl DiscordDisplay for BobError {
    fn to_discord(&self) -> String {
        let emoji = match &self.knd {
            ErrorKind::User      => "⚠️",
            ErrorKind::Admin     => "⛔️",
            ErrorKind::Developer => "🐛",
            ErrorKind::External  => "🌐",
        };

        let msg = match &self.msg {
            None    => "",
            Some(m) => m,
        };

        let code = match &self.err {
            None    => "",
            Some(e) => format!("```\n{}\n```", &e)
        };

        format!(indoc!{"
            {emoji} {msg}
            {code}
        "}, emoji=&emoji, msg=&msg, code=&code)
    }
}

impl<T: Error> From<T> for BobError {
    fn from(err: T) -> Self {
        BobError {
            msg: None,
            err: Some(err),
            knd: ErrorKind::Developer
        }
    }
}

pub type BobResult<T> = Result<T, BobError>;

pub trait IntoBobError<T> {
    fn map_bob(self, knd: ErrorKind, msg: &str) -> BobResult<T>;
}

impl<T, E> IntoBobError<T> for Result<T, E> {
    fn map_bob(self, knd: ErrorKind, msg: &str) -> BobResult<T> {
        self.map_err(|err|
            BobError {
                knd: knd,
                err: Some(err),
                msg: Some(String::from(msg))
            }
        )
    }
}

impl<T> IntoBobError<T> for Option<T> {
    fn map_bob(self, knd: ErrorKind, msg: &str) -> BobResult<T> {
        self.ok_or_else(||
            BobError {
                knd: knd,
                err: None,
                msg: Some(String::from(msg))
            }
        )
    }
}