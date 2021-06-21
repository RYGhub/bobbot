use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;
use crate::tasks::build::task_build;
use crate::extensions::*;
use crate::args::{BobArgs};
use crate::tasks::mov::task_move;
use crate::errors::{BobResult, ErrorKind};
use std::convert::Infallible;


#[command]
#[aliases("b")]
#[only_in(guilds)]
pub async fn build(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    debug!("Called command: build");

    match true_build(&ctx, &msg, args).await {
        Ok(_) => {},
        Err(e) => {
            e.handle(&ctx.http, &msg).await;
        }
    };

    CommandResult::Ok(())
}


async fn true_build(ctx: &Context, msg: &Message, args: Args) -> BobResult<()> {
    let guild = msg.bob_guild_id().await?.clone();
    let guild = guild.bob_partial_guild(&ctx.http).await?;
    let name = args.channel_name()?;

    let category = msg.channel_id
        .bob_guild_channel(&ctx.http)
        .await?
        .bob_category(&ctx.http)
        .await?;

    let channel = task_build(&ctx, &guild, &name, &category, None).await?;
    task_move(&ctx, &guild, &msg.author.id, &channel.id).await?;

    Ok(())
}