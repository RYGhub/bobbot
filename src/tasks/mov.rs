//! This module contains a task to move an user from a voice channel to another.

use serenity::model::prelude::{PartialGuild, UserId, ChannelId, Member};
use serenity::prelude::{Context};
use crate::errors::{BobResult, BobCatch, ErrorKind};

/// Move an [UserId] to a voice [ChannelId].
pub async fn task_move(ctx: &Context, guild: &PartialGuild, user_id: &UserId, channel_id: &ChannelId) -> BobResult<Member> {
    guild.move_member(
        &ctx.http,
        user_id.clone(),
        channel_id.clone(),
    ).await.bob_catch(ErrorKind::Admin, "Couldn't move member to voice channel")
}