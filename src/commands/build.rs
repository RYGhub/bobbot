use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::extensions::*;
use crate::errors::*;
use crate::tasks::build::task_build;
use crate::tasks::mov::task_move;
use crate::tasks::clean::task_clean;
use crate::utils::channel_names::{Channelizable};


pub async fn command_build(ctx: &Context, guild_id: &GuildId, channel_id: &ChannelId, member: &Member, data: &ApplicationCommandInteractionData) -> BobResult<String> {
    debug!("Called command: build");

    let guild = guild_id.bob_partial_guild(&ctx.http).await?;
    let category = channel_id
        .bob_guild_channel(&ctx.http).await?
        .bob_category(&ctx.http).await?;

    let options = data.to_owned().option_hashmap();
    let name = options.arg_req_string("name")?.channelify();
    let preset = options.arg_opt_string("preset")?;

    let created = task_build(
        &ctx, &guild, &name, &category,
        &preset.as_ref().map(|s| s.as_str())
    ).await?;

    let result = task_move(&ctx, &guild, &member.user.id, &created.id).await
        .bob_catch(ErrorKind::Admin, "Couldn't move user to temporary voice channel")
        .map(|_| {
            format!("🔨 Built temporary voice channel {}!", &created.mention())
        });


    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            let _ = created.delete(&ctx.http)
                .await.bob_catch(ErrorKind::Admin, "Couldn't undo channel creation.")?;

            Err(BobError::from_msg(ErrorKind::User, "You're not connected to voice chat!"))
        }
    }
}
