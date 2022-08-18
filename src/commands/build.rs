use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::application::interaction::application_command::CommandData;
use crate::extensions::*;
use crate::errors::*;
use crate::tasks::build::task_build;
use crate::tasks::mov::task_move;
use crate::utils::channel_names::{Channelizable};


pub async fn command_build(ctx: &Context, guild_id: GuildId, channel_id: ChannelId, member: &Member, data: &CommandData) -> BobResult<String> {
    debug!("Called command: build");

    let guild = guild_id.ext_partial_guild(&ctx.http).await?;
    let category = channel_id
        .ext_guild_channel(&ctx.http).await?
        .ext_category(&ctx.http).await?;

    let options = data.to_owned().options.option_hashmap();
    let name = options.req_string("name")?.channelify();
    let preset = options.opt_string("preset")?.map(|p| p.channelify());

    let created = task_build(
        ctx, &guild, &name, member, &category,
        &preset.as_deref()
    ).await?;

    let _ = task_move(ctx, &guild, member.user.id, created.id).await;

    Ok(format!("🔨 Built temporary voice channel {}!", &created.mention()))
}
