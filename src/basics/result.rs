use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::{Error};
use serenity::framework::standard::{CommandError};


#[derive(Debug, Clone)]
pub struct BobError {
    pub msg: &'static str,
}

impl Display for BobError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "**Error**: {}", &self.msg)
    }
}

impl Error for BobError {}


pub type BobResult<T> = Result<T, BobError>;


pub fn convert_error<T>(error: T, msg: &'static str) -> BobError
    where T: Error + Display,
{
    error!("Error: {:?}", &error);
    BobError {msg}
}
