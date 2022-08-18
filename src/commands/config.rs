use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::interactions::application_command::{ApplicationCommandInteractionDataOption};
use crate::extensions::*;
use crate::errors::*;
use crate::database::models::{WithDeletionTime, WithCommandChannel};
use std::time::Duration;


pub async fn command_config_cc(_ctx: &Context, guild_id: GuildId, _channel_id: ChannelId, member: &Member, data: &Vec<ApplicationCommandInteractionDataOption>) -> BobResult<String> {
    debug!("Called command: config cc");

    let options = data.to_owned().option_hashmap();

    let channel = options.req_channel("channel")?;
    let permissions = member.permissions
        .bob_catch(ErrorKind::External, "Interaction didn't have the Member's Permissions")?;

    if !permissions.manage_channels() {
        return Err(BobError::from_msg(ErrorKind::User, "You need to have **Manage Channels** permission on the guild to change the Command Channel."))
    }

    if channel.kind != ChannelType::Text {
        return Err(BobError::from_msg(ErrorKind::User, "Only Text Channels are valid Command Channels."))
    }

    guild_id.set_command_channel(channel.id)?;

    Ok(format!("🔧 Command channel set to {}!", &channel.id.mention()))
}


pub async fn command_config_dt(_ctx: &Context, guild_id: GuildId, _channel_id: ChannelId, member: &Member, data: &Vec<ApplicationCommandInteractionDataOption>) -> BobResult<String> {
    debug!("Called command: dt");

    let options = data.to_owned().option_hashmap();

    let timeout = options.req_integer("timeout")?;
    let permissions = member.permissions
        .bob_catch(ErrorKind::External, "Interaction didn't have the Member's Permissions")?;

    if !permissions.manage_guild() {
        return Err(BobError::from_msg(ErrorKind::User, "You need to have **Manage Guild** permission on the guild to change the Deletion Time."))
    }

    guild_id.set_deletion_time(Duration::from_secs(timeout.unsigned_abs()))?;

    Ok(format!("🔧 Deletion time set to **{} seconds**!", &timeout))
}