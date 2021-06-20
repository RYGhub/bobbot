use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::errors::*;
use crate::extensions::*;


/// Check if the user who sent the message is an *Administrator* of the server.
pub async fn check_administrator(ctx: &Context, msg: &Message) -> CheckResult {
    let guild = msg.bob_guild_id().await?.bob_guild(&ctx.cache).await?;

    let member = guild.bob_member(&ctx.http, msg.author.id.clone()).await?;
    let permissions = member.permissions(&ctx).await
        .bob_catch(ErrorKind::External, "Couldn't retrieve server member permissions.")?;

    match permissions.administrator() {
        false => Err(
            BobError::from_msg(
                ErrorKind::User,
                "You are not an Administrator of this server."
            )
        ),
        true => Ok(())
    }
}
