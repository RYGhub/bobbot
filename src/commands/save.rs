use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::application::interaction::application_command::{CommandData};
use crate::extensions::*;
use crate::errors::*;
use crate::database::models::{IntoPresetData, DatabaseAction};
use crate::utils::channel_names::{Channelizable};


pub async fn command_save(ctx: &Context, _guild_id: GuildId, _channel_id: ChannelId, member: &Member, data: &CommandData) -> BobResult<String> {
    debug!("Called command: save");

    let options = data.to_owned().options.option_hashmap();
    let preset = options.req_string("preset")?.channelify();
    let template = options.req_channel("template")?.id.ext_guild_channel(&ctx.http).await?;
    let overwrite = options.opt_boolean("overwrite")?.unwrap_or(false);

    let permissions = member.permissions
        .bob_catch(ErrorKind::External, "Interaction didn't have the Member's Permissions")?;

    if overwrite && !permissions.manage_channels() {
        return Err(BobError::from_msg(ErrorKind::User, "You need to have **Manage Channels** permission on the guild to overwrite an existing preset."));
    };

    let action = template.save_as_preset(preset.clone(), overwrite)?;

    match action {
        DatabaseAction::Created(_) => {
            Ok(format!("📀 Preset `{}` created successfully from {}!", &preset, &template.mention()))
        }
        DatabaseAction::Updated(_) => {
            Ok(format!("💿 Preset `{}` overwritten successfully with {}!", &preset, &template.mention()))
        }
        DatabaseAction::Deleted => {
            Err(BobError::from_msg(ErrorKind::Developer, "Saving resulted in the deletion of a preset"))
        }
        DatabaseAction::None => {
            Err(BobError::from_msg(ErrorKind::User, "A preset with that name already exists."))
        }
    }
}
