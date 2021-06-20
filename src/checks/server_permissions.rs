use serenity::model::prelude::{Message};
use serenity::prelude::{Context};
use crate::errors::{BobResult, user_error};


/// Check if the user who sent the message is an *Administrator* of the server.
pub async fn check_administrator(ctx: &Context, msg: &Message) -> BobResult<()> {
    let guild = msg.guild(&ctx.cache).await?;
    let member = guild.member(&ctx.cache, &msg.author.id).await?;
    let permissions = member.permissions(&ctx.cache).await?;

    match permissions.administrator() {
        false => Err(user_error("You are not an *Administrator* of this server.")),
        true => Ok(())
    }
}
