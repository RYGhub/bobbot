pub mod build;
pub mod save;
pub mod load;

use serenity::framework::standard::macros::*;

use crate::commands::bob::build::BUILD_COMMAND;
use crate::commands::bob::save::SAVE_COMMAND;
use crate::commands::bob::load::LOAD_COMMAND;

#[group]
#[commands(build, save, load)]
pub struct Bob;
