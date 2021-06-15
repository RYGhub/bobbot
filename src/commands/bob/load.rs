use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::basics::*;


/// Build a new temporary channel with the specified preset.
#[command]
#[aliases("l")]
#[only_in(guilds)]
pub async fn load(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Running command: !load");

    let guild = get_guild(&msg, &ctx.cache).await;
    let command_channel = get_channel(&msg, &ctx.cache).await;
    let category = get_category(&command_channel, &ctx.http).await;
    broadcast_typing(&command_channel, &ctx.http);

    let preset_name = parse_preset_name(&mut args);
    let channel_name = parse_channel_name(args);
    let base_permow = get_category_permows(&category);
    let own_permow = get_own_permow(&ctx.cache).await;
    let author_permow = get_author_permow(&msg);

    let preset = read_guild_preset(&guild.id, &preset_name);

    let created = create_channel(
        &ctx.http,
        &guild,
        &category.id,
        &channel_name,
        [
            base_permow,
            preset.permows,
            vec![own_permow],
            vec![author_permow]
        ].concat(),
        preset.bitrate,
        preset.user_limit,
    ).await;

    move_member(&ctx.http, &guild, &msg.author.id, &created.id).await;

    info!("Successfully built channel #{} for {}#{} with preset {}!",
          &created.name, &msg.author.name, &msg.author.discriminator, &preset_name);

    msg.channel_id.say(
        &ctx.http,
        format!(
            "🔨 Built channel {} with owner {} from preset `{}`!",
            &created.mention(),
            &msg.author.mention(),
            &preset_name)
    ).await;

    Ok(())
}
