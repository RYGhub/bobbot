use core::fmt::Display;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;

pub fn create_temp_channel<S, I>(ctx: &mut Context, guild: &Guild, category_id: &ChannelId, name: S, permissions: I) -> std::result::Result<GuildChannel, CommandError>
    where S: Display, I: IntoIterator<Item=PermissionOverwrite>
{
    let created = guild.create_channel(&ctx.http, |c| {
        debug!("Temp channel name will be: {}", &name);
        c.name(name);

        debug!("Temp channel type will be: Voice");
        c.kind(ChannelType::Voice);

        debug!("Temp channel category will be: {}", &category_id);
        c.category(category_id.clone());

        debug!("Temp channel permissions will use the ones moved here as a parameter");
        c.permissions(permissions)
    })?;
    info!("Created temp channel #{}", &created.name);

    Ok(created)
}
