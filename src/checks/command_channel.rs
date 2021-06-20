use serenity::model::prelude::{Message};
use crate::database::models::{CommandChannel};
use crate::errors::{BobResult, user_error, bot_msg_error};


/// Check if the message was sent in the Command Channel.
pub async fn check_in_command_channel(msg: &Message) -> BobResult<()> {
    let guild = msg.guild_id
        .ok_or_else(|| bot_msg_error("Couldn't get guild_id"))?;

    let cc_opt = CommandChannel::get(&guild.id)?;

    match cc_opt {
        None => {
            Err(user_error("This server has not yet set a Command Channel."))
        },
        Some(cc_id) => {
            match msg.channel_id == cc_id {
                true => Ok(()),
                false => Err(user_error("This message was not sent in the Command Channel."))
            }
        },
    }
}
