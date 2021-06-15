use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::{Error};


#[derive(Debug, Clone)]
pub struct BobError {
    pub msg: &'static str
}

impl Display for BobError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "**Error**: {}", &self.msg)
    }
}

impl Error for BobError {}


pub type BobResult<T> = Result<T, BobError>;
