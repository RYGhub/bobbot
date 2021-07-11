use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::extensions::*;
use crate::errors::*;
use crate::tasks::build::task_build;
use crate::tasks::mov::task_move;
use crate::tasks::clean::task_clean;
use crate::utils::channel_names::{Channelizable};
use crate::database::models::{WithDeletionTime, WithCommandChannel};
use std::time::Duration;


pub async fn command_config_cc(ctx: &Context, guild_id: &GuildId, channel_id: &ChannelId, member: &Member, data: &Vec<ApplicationCommandInteractionDataOption>) -> BobResult<String> {
    let options = data.to_owned().option_hashmap();

    let channel = options.req_channel("channel")?;
    let permissions = member.permissions
        .bob_catch(ErrorKind::Developer, "Interaction didn't have the Member's Permissions")?;

    if !permissions.manage_channels() {
        return Err(BobError::from_msg(ErrorKind::User, "You need to have **Manage Channels** permission on the guild to change the Command Channel."))
    }

    if channel.kind != ChannelType::Text {
        return Err(BobError::from_msg(ErrorKind::User, "Only Text Channels are valid Command Channels."))
    }

    guild_id.set_command_channel(channel.id.clone());

    Ok(format!("🔧 Command channel set to {}!", &channel.id.mention()))
}


pub async fn command_config_dt(ctx: &Context, guild_id: &GuildId, channel_id: &ChannelId, member: &Member, data: &Vec<ApplicationCommandInteractionDataOption>) -> BobResult<String> {
    let options = data.to_owned().option_hashmap();

    let timeout = options.req_integer("timeout")?;
    let permissions = member.permissions
        .bob_catch(ErrorKind::Developer, "Interaction didn't have the Member's Permissions")?;

    if !permissions.manage_guild() {
        return Err(BobError::from_msg(ErrorKind::User, "You need to have **Manage Guild** permission on the guild to change the Deletion Time."))
    }

    guild_id.set_deletion_time(Duration::from_secs(timeout.unsigned_abs().clone()));

    Ok(format!("🔧 Deletion time set to **{} seconds**!", &timeout))
}