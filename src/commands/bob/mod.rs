pub mod build;
pub mod save;
pub mod load;
pub mod list;

use serenity::framework::standard::macros::*;

use crate::commands::bob::build::*;
use crate::commands::bob::save::*;
use crate::commands::bob::load::*;
use crate::commands::bob::list::*;

#[group]
#[commands(build, save, load, list)]
pub struct Bob;
