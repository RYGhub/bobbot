pub mod build;

use serenity::framework::standard::macros::*;
use self::build::*;


#[group]
#[commands(build)]
pub struct Bob;
