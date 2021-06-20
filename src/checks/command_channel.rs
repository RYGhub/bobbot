use serenity::model::prelude::{Message};
use crate::database::models::{WithCommandChannel};
use crate::errors::*;


/// Check if the message was sent in the Command Channel.
pub async fn check_in_command_channel(msg: &Message) -> CheckResult {
    let guild_id = msg.guild_id
        .bob_catch(ErrorKind::External, "Failed to get the id of the current server.")?;

    let cc_id = guild_id.get_command_channel()
        .bob_catch(ErrorKind::External, "Failed to get the id of the command channel.")?
        .bob_catch(ErrorKind::User, "This server has not yet set a Command Channel.")?;

    match msg.channel_id == cc_id {
        true => Ok(()),
        false => Err(
            BobError {
                knd: ErrorKind::User,
                msg: Some(String::from("This message was not sent in the Command Channel.")),
                err: None
            }
        )
    }
}
