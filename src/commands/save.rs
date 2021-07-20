use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::extensions::*;
use crate::errors::*;
use crate::tasks::build::task_build;
use crate::tasks::mov::task_move;
use crate::tasks::clean::task_clean;
use crate::utils::channel_names::{Channelizable};
use crate::database::models::{IntoPresetData, Preset};


pub async fn command_save(ctx: &Context, guild_id: &GuildId, _channel_id: &ChannelId, member: &Member, data: &ApplicationCommandInteractionData) -> BobResult<String> {
    debug!("Called command: save");

    let guild = guild_id.ext_partial_guild(&ctx.http).await?;

    let options = data.to_owned().options.option_hashmap();
    let preset = options.req_string("preset")?.channelify();
    let template = options.req_channel("template")?.id.ext_guild_channel(&ctx.http).await?;
    let overwrite = options.opt_boolean("overwrite")?.unwrap_or_else(false);

    let permissions = member.permissions
        .bob_catch(ErrorKind::External, "Interaction didn't have the Member's Permissions")?;

    if overwrite && !permissions.manage_channels() {
        Err(BobError::from_msg(ErrorKind::User, "You need to have **Manage Channels** permission on the guild to overwrite an existing preset."))
    }

    template.save_as_preset(preset.clone(), overwrite);

    Ok(format!("💾 Channel {} saved successfully into preset `{}`!", &template.mention(), &preset))
}
