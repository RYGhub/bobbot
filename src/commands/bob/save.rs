use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::basics::*;


/// Save the permissions to a file.
#[command]
#[aliases("s")]
#[only_in(guilds)]
pub async fn save(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Running command: !save");

    let guild = get_guild(&msg, &ctx.cache).await;
    let command_channel = get_channel(&msg, &ctx.cache).await;
    let category = get_category(&command_channel, &ctx.http).await;
    broadcast_typing(&command_channel, &ctx.http);

    let author_voice_channel = get_user_voice_channel(&ctx.http, &guild, &msg.author.id).await;
    let preset_name = parse_preset_name(&mut args);
    let preset = get_author_preset(&ctx.http, &guild, &msg).await;

    write_guild_preset(&guild.id, &preset_name, &preset);

    info!("Successfully created preset {}!", &preset_name);
    msg.channel_id.say(&ctx.http, format!("📁 Saved permissions of {} to preset `{}`!", &author_voice_channel.mention(), &preset_name)).await;

    Ok(())
}
