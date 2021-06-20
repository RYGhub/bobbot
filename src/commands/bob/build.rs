use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::basics::command::{get_guild, get_channel, broadcast_typing, get_permows, reply};
use crate::basics::channel::{get_category, create};
use crate::basics::voice::{move_member};
use crate::basics::args::{parse_channel_name};


/// Build a new temporary channel.
#[command]
#[aliases("b")]
#[only_in(guilds)]
pub async fn build(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    debug!("Running command: !build");

    let guild = get_guild(&ctx.cache, &msg).await?;
    let channel = get_channel(&ctx.cache, &msg).await?;
    let category = get_category(&ctx.http, &channel).await?;

    broadcast_typing(&ctx.http, &channel).await?;
    let channel_name = parse_channel_name(args)?;

    let created = create(
        &ctx.http,
        &guild,
        category.clone().and_then(|cat| Some(cat.id)),  // Can be improved, I think
        &channel_name,
        get_permows(&ctx.cache, &category, msg.author.id.clone()).await,
        64000,
        None,
    ).await?;

    move_member(&ctx.http, &guild, &msg.author.id, &created.id).await?;

    reply(
        &ctx.http, &msg,
        format!(
            "🔨 Built channel {} with owner {}!",
            &created.mention(),
            &msg.author.mention()
        )
    ).await?;

    Ok(())
}
