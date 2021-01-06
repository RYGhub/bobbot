use core::fmt::Display;
use num_traits::ToPrimitive;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;

pub async fn create_temp_channel<S, I>(ctx: &Context, guild: &Guild, category_id: &ChannelId, name: S, permissions: I, bitrate: &u64, user_limit: &Option<u64>) -> std::result::Result<GuildChannel, CommandError>
    where S: Display, I: IntoIterator<Item=PermissionOverwrite>
{
    let created = guild.create_channel(&ctx.http, |c| {
        debug!("Channel name will be: {}", &name);
        c.name(name);

        debug!("Channel type will be: Voice");
        c.kind(ChannelType::Voice);

        debug!("Channel category will be: {}", &category_id);
        c.category(category_id.clone());

        debug!("Channel permissions will use the ones moved here as a parameter");
        c.permissions(permissions);

        // why tf does this take u32 in input when it is a u64 in output
        debug!("Channel bitrate will be: {}", &bitrate);
        c.bitrate(bitrate.clone().to_u32().expect("Bitrate could not be converted to u32"));

        // why tf does this take u32 in input when it is a Option<u64> in output
        if let Some(limit) = user_limit {
            debug!("Channel user limit will be: {}", &limit);
            c.user_limit(limit.clone().to_u32().expect("User limit could not be converted to u32"));
        }
        else {
            debug!("Channel won't have a user limit")
        }

        c
    }).await?;
    info!("Created temp channel #{}", &created.name);

    Ok(created)
}
