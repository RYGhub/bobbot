use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::checks::sent_in_bob::*;
use crate::checks::bob_has_category::*;
use crate::checks::author_connected_to_voice::*;

use crate::utils::kebabify;
use crate::utils::create_temp_channel::create_temp_channel;


#[command]
#[description="Build a new temporary channel."]
#[aliases("b")]
#[only_in(guilds)]
#[checks(SentInBob, BobHasCategory, AuthorConnectedToVoice)]
pub async fn build(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    debug!("Running command: !build");

    debug!("Getting guild...");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    debug!("Guild is: {}", &guild.name);

    debug!("Getting command channel...");
    let command_channel = msg.channel(&ctx.cache).await.unwrap().guild().unwrap();
    debug!("Command channel is: #{}", &command_channel.name);

    debug!("Getting category...");
    let category_channel = command_channel.category_id.unwrap().to_channel(&ctx.http).await.unwrap().category().unwrap();
    debug!("Category channel is: #{}", &category_channel.name);

    debug!("Parsing args...");
    let new_channel_name = kebabify(args.rest());
    debug!("Channel name will be: #{}", &new_channel_name);

    debug!("Broadcasting typing...");
    command_channel.broadcast_typing(&ctx.http).await?;

    debug!("Getting default channel permissions from the Bob category...");
    let mut permissions = category_channel.permission_overwrites.clone();

    debug!("Adding full permissions for channel owner: {}", &msg.author.mention());
    permissions.push(PermissionOverwrite{
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(msg.author.id.clone())
    });

    debug!("Creating channel...");
    let bitrate = 64000;
    let user_limit = None;
    let created = create_temp_channel(&ctx, &guild, &category_channel.id, &new_channel_name, permissions, &bitrate, &user_limit).await?;

    debug!("Sending channel created message...");
    msg.channel_id.say(&ctx.http, format!("🔨 Built channel {} with owner {}!", &created.mention(), &msg.author.mention())).await?;

    debug!("Moving command caller to the created channel...");
    guild.move_member(&ctx.http, &msg.author.id, &created.id).await?;

    info!("Successfully built channel #{} for {}#{}!", &created.name, &msg.author.name, &msg.author.discriminator);
    Ok(())
}
