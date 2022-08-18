use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::error::{Error};
use serenity::model::prelude::{Message};
use serenity::http::Http;
use indoc::indoc;
use crate::utils::discord_display::{DiscordDisplay};


/// The four possible "causes" of an error:
/// - `ErrorKind::User`: if an error is caused by a mistake on the bot user's part.
/// - `ErrorKind::Admin`: if an error is caused by misconfiguration by the Discord server admin.
/// - `ErrorKind::Host`: if an error is caused by misconfiguration by the bot hosting provider.
/// - `ErrorKind::Developer`: if an error is caused by a wrong assumption made by the bot developer.
/// - `ErrorKind::External`: if an error in an external service occurred, and nothing can be done about it.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    User,
    Admin,
    Host,
    Developer,
    External,
}

/// A structure describing an error that occurred during the bot's operation.
///
/// It must include a [ErrorKind] (`knd`), and may include a message (`msg`) and/or an object implementing [Error]
/// (`err`).
#[derive(Debug)]
pub struct BobError {
    pub knd: ErrorKind,
    pub msg: Option<String>,
    pub err: Option<Box<dyn Error + Send + Sync>>,
}

impl BobError {
    /// Create a new [BobError] given a [ErrorKind] and a message.
    pub fn from_msg(knd: ErrorKind, msg: &str) -> Self {
        BobError {
            knd,
            msg: Some(String::from(msg)),
            err: None,
        }
    }

    pub async fn handle(&self, http: &Http, msg: &Message) -> BobResult<Message> {
        match &self.knd {
            ErrorKind::User => {
                debug!("{}", &self);
            },
            ErrorKind::Admin => {
                debug!("{}", &self);
            },
            ErrorKind::Host => {
                error!("{}", &self);
            },
            ErrorKind::Developer => {
                error!("{}", &self);
            },
            ErrorKind::External => {
                warn!("{}", &self);
            },
        }

        msg.reply(&http, &self.to_discord().to_string()).await
            .bob_catch(ErrorKind::Admin, "Couldn't handle error")
    }
}

impl Display for BobError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.msg.is_some() && self.err.is_some() {
            write!(f, "{:?}: {}", &self.err.as_ref().unwrap(), &self.msg.as_ref().unwrap())
        }
        else if self.msg.is_some() {
            write!(f, "Error: {}", &self.msg.as_ref().unwrap())
        }
        else if self.err.is_some() {
            write!(f, "{:?}: _no message_", &self.err.as_ref().unwrap())
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
            ErrorKind::Host      => "☢️",
            ErrorKind::Developer => "🐛",
            ErrorKind::External  => "🌐",
        };

        let msg = match &self.msg {
            None    => "",
            Some(m) => m,
        };

        let code = match &self.err {
            None    => "".to_string(),
            Some(e) => format!("```\n{}\n```", &e)
        };

        format!(indoc!{"
            {emoji} {msg}
            {code}
        "}, emoji=&emoji, msg=&msg, code=&code)
    }
}

/// A [Result] that always returns a [BobError] as [Result::Err].
pub type BobResult<T> = Result<T, BobError>;


/// A trait denoting that an object can be converted in a [BobResult] with a certain [ErrorKind] and message.
///
/// Implemented for [Result] and [Option].
pub trait BobCatch<T> {
    /// Convert the current object into a [BobResult] with the specified [ErrorKind] and message.
    fn bob_catch(self, knd: ErrorKind, msg: &str) -> BobResult<T>;
}

impl<T, E: Error + Send + Sync + 'static> BobCatch<T> for Result<T, E> {
    fn bob_catch(self, knd: ErrorKind, msg: &str) -> BobResult<T> {
        self.map_err(|err| BobError {
            knd,
            err: Some(Box::from(err)),
            msg: Some(String::from(msg))
        })
    }
}

impl<T> BobCatch<T> for Option<T> {
    fn bob_catch(self, knd: ErrorKind, msg: &str) -> BobResult<T> {
        self.ok_or_else(||
            BobError {
                knd,
                err: None,
                msg: Some(String::from(msg))
            }
        )
    }
}