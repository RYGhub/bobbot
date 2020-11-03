use std::env;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;


#[check]
#[name = "PresetExists"]
pub async fn check_preset_exists(ctx: &Context, msg: &Message, args: &mut Args) -> CheckResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let preset_name = args.current();
    if preset_name.is_none() {
        return CheckResult::new_user("You didn't specify the preset you wanted to load.")
    }
    let preset_name = preset_name.unwrap();

    let current_path = env::current_dir().expect("Could not get current working directory");
    let presets_dir = current_path.join("presets");
    let guild_dir = presets_dir.join(format!("{}", &guild.id));
    let preset_path = guild_dir.join(format!("{}.toml", &preset_name));

    match preset_path.exists() {
        true => CheckResult::Success,
        false => CheckResult::new_user(format!("The preset {} does not exist.", &preset_name)),
    }
}