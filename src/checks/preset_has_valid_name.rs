use std::env;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;


#[check]
#[name = "PresetHasValidName"]
pub fn check_preset_exists(_ctx: &mut Context, _msg: &Message, args: &mut Args) -> CheckResult {
    let preset_name = args.current();
    if preset_name.is_none() {
        return CheckResult::new_user("You didn't specify the name to save the preset to.")
    }

    CheckResult::Success
}