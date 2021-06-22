//! This module contains a task to move an user from a voice channel to another.

use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::errors::*;

/// Move an [UserId] to a voice [ChannelId].
pub async fn task_move(ctx: &Context, guild: &PartialGuild, user_id: &UserId, channel_id: &ChannelId) -> BobResult<Member> {
    debug!("Running task: move | In <G:{}>, move <@{}> to <#{}>", &guild.name, &user_id, &channel_id);

    guild.move_member(
        &ctx.http,
        user_id.clone(),
        channel_id.clone(),
    ).await.map_err(|err| {
        // This is awful
        if format!("{}", &err).contains("Target user is not connected to voice.") {
            BobError::from_msg(ErrorKind::User, "You're not connected to voice chat!")
        }
        else {
            BobError::from_msg(ErrorKind::Admin, "Could't move user to the newly created channel.")
        }
    })
}