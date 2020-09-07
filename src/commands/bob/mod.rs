pub mod build;
pub mod save;
pub mod load;

use serenity::framework::standard::macros::*;

use crate::commands::bob::build::*;
use crate::commands::bob::save::*;
use crate::commands::bob::load::*;

#[group]
#[commands(build, save, load)]
pub struct Bob;
