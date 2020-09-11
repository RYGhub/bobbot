use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::checks::sent_in_bob::*;
use crate::checks::bob_has_category::*;
use crate::checks::author_connected_to_voice::*;

use crate::utils::kebabify;
use crate::utils::create_temp_channel::create_temp_channel;


/// Build a new temporary channel.
#[command]
#[only_in(guilds)]
#[checks(SentInBob, BobHasCategory, AuthorConnectedToVoice)]
pub fn build(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    debug!("Running command: !build");

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild = guild.read();
    let channel = msg.channel(&ctx.cache).unwrap().guild().unwrap();
    let channel = channel.read();
    let category = channel.category_id.unwrap().to_channel(&ctx.http).unwrap().category().unwrap();
    let category = category.read();

    let new_channel_name = kebabify(args.rest());

    debug!("Starting to type");
    channel.broadcast_typing(&ctx.http)?;

    debug!("Cloning channel permissions from the bob category");
    let mut permissions = category.permission_overwrites.clone();
    permissions.push(PermissionOverwrite{
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(msg.author.id.clone())
    });

    debug!("Creating temp channel");
    let created = create_temp_channel(ctx, &guild, &category.id, &new_channel_name, permissions)?;

    debug!("Sending channel created message");
    msg.channel_id.say(&ctx.http, format!("🔨 Temp channel <#{}> was built.", &created.id))?;

    debug!("Moving command caller to the created channel");
    guild.move_member(&ctx.http, &msg.author.id, &created.id)?;

    debug!("Build command executed successfully!");

    Ok(())
}