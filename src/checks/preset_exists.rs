use std::env;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;


#[check]
#[name = "PresetExists"]
pub async fn check_preset_exists(ctx: &Context, msg: &Message, args: &mut Args) -> CheckResult {
    debug!("Running check: PresetExists");

    debug!("Getting guild...");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    debug!("Guild is: {}", &guild.name);

    debug!("Parsing args...");
    let preset_name = args.current();
    if preset_name.is_none() {
        return CheckResult::new_user("You must specify the preset you want to load.")
    }
    let preset_name = preset_name.unwrap();
    debug!("Preset name is: {}", &preset_name);

    debug!("Getting working directory...");
    let current_path = env::current_dir().expect("Could not get working directory");

    let presets_dir = current_path.join("presets");
    debug!("Accessing presets directory: {}", &presets_dir.to_string_lossy());

    let guild_dir = presets_dir.join(format!("{}", guild.id));
    debug!("Accessing guild presets directory: {}", &guild_dir.to_string_lossy());

    let file_name = format!("{}.toml", &preset_name);
    let preset_path = guild_dir.join(&file_name);
    debug!("Accessing guild preset file: {}", &preset_path.to_string_lossy());

    match preset_path.exists() {
        true => CheckResult::Success,
        false => CheckResult::new_user(format!("The preset `{}` does not exist.", &preset_name)),
    }
}