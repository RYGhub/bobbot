use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

#[allow(unused_imports)]
use crate::checks::sent_in_bob::SENTINBOB_CHECK;
#[allow(unused_imports)]
use crate::checks::bob_has_category::BOBHASCATEGORY_CHECK;
#[allow(unused_imports)]
use crate::checks::author_connected_to_voice::AUTHORCONNECTEDTOVOICE_CHECK;

use crate::utils::kebabify;


/// Build a new temporary channel.
#[command]
#[only_in(guilds)]
#[checks(SentInBob)]
#[checks(BobHasCategory)]
#[checks(AuthorConnectedToVoice)]
pub fn build(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    debug!("Running command: !build");

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild = guild.read();
    let channel = msg.channel(&ctx.cache).unwrap().guild().unwrap();
    let channel = channel.read();
    let category = channel.category_id.unwrap().to_channel(&ctx.http).unwrap().category().unwrap();
    let category = category.read();

    let new_channel_name = kebabify(args.rest());

    channel.broadcast_typing(&ctx.http)?;
    debug!("Started typing");

    let created = guild.create_channel(&ctx.http, |c| {
        debug!("Temp channel name will be: {}", &new_channel_name);
        c.name(new_channel_name);

        debug!("Temp channel type will be: Voice");
        c.kind(ChannelType::Voice);

        debug!("Temp channel category will be: {}", &channel.category_id.unwrap());
        c.category(&channel.category_id.unwrap());

        debug!("Temp channel permissions will be cloned from the category");
        let mut permissions = category.permission_overwrites.clone();
        permissions.push(PermissionOverwrite{
            allow: Permissions::all(),
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(msg.author.id.clone())
        });
        c.permissions(permissions)
    })?;
    info!("Created temp channel #{}", &created.name);

    msg.channel_id.say(&ctx.http, format!("🔨 Temp channel <#{}> was built.", &created.id))?;
    debug!("Sent channel created message");

    guild.move_member(&ctx.http, &msg.author.id, &created.id)?;
    debug!("Moved command caller to the created channel");

    Ok(())
}