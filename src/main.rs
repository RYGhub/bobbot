#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::env;
use std::result;
use serenity::Client;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::StandardFramework;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;
use regex::Regex;

struct BobHandler;

#[group]
#[commands(build)]
struct Bob;


/// Initialize and start the bot.
fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    env::var("BOB_CHANNEL_NAME").expect("Missing BOB_CHANNEL_NAME");

    pretty_env_logger::init();
    debug!("Logger initialized!");

    let mut client = Client::new(&token, BobHandler).expect("Error creating Discord client");
    debug!("Discord client created!");

    client.with_framework(
        StandardFramework::new().configure(
            |c| c
                .prefix("!")
        )
        .group(&BOB_GROUP)
        .on_dispatch_error(error)
    );
    debug!("Client framework initialized!");

    client.start_autosharded().expect("Error starting Discord client");
    debug!("Autosharded Discord client started!");
}


impl EventHandler for BobHandler {

    /// Handle the ready event.
    fn ready(&self, _context: Context, ready: Ready) {
        info!("{} is ready!", &ready.user.name);
    }

    /// Called when the voice state of an user changes.
    // IntelliJ Rust inspection is broken
    // https://github.com/intellij-rust/intellij-rust/issues/1191
    // noinspection RsTraitImplementation
    fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        match handle_voice_state_update(ctx, guild_id, old, new) {
            Err(s) => {
                debug!("Skipping channel deletion: {}", s);
            }
            _ => (),
        }
    }

}

/// Handle command errors.
fn error(ctx: &mut Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::OnlyForGuilds => {
            debug!("Rejecting command sent outside of a guild");
            let _ = msg.reply(&ctx.http, "This command only works in a guild.");
        }
        _ => {
            warn!("Unmatched error occoured!");
            let _ = msg.reply(&ctx.http, "Unmatched error occoured.");
        }
    }
}

/// Convert a string to **kebab-case**.
fn sanitize_channel_name(s: &str) -> String {
    lazy_static! {
        static ref REPLACE_PATTERN: Regex = Regex::new("[^a-z0-9]").unwrap();
    }

    let mut last = s.len();
    if last > 100 {
        last = 100;
    }

    let s = &s[..last];
    let s = s.to_ascii_lowercase();
    let s: String = REPLACE_PATTERN.replace_all(&s, " ").into_owned();
    let s = s.trim();
    let s = s.replace(" ", "-");

    debug!("Sanitized channel name to: {}", &s);
    s
}


#[command]
#[only_in(guilds)]
/// Build a new temporary channel.
fn build(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).expect("Could not fetch guild info.");
    let guild = guild.read();
    debug!("Build called in guild: {}", &guild.name);

    let channel = &msg
        .channel(&ctx.cache).expect("Could not fetch channel info.")
        .guild().unwrap();
    let channel = channel.read();
    debug!("Build called in channel: {}", &channel.name);

    let bob_channel_name = env::var("BOB_CHANNEL_NAME").unwrap();
    if channel.name != bob_channel_name {
        info!("Refusing to build for a command called in a channel with a name that doesn't match: {}", bob_channel_name);
        &msg.reply(&ctx.http, "Bob cannot run in this channel.")?;
        return Ok(());
    }

    let category_id = match channel.category_id {
        None => {
            error!("Bob channel is not in a category!");
            msg.reply(&ctx.http, "This channel is not in a category!")?;
            return Ok(());
        },
        Some(category_id) => category_id,
    };
    let category = category_id.to_channel(&ctx.http).expect("Could not fetch category info.").category().unwrap();
    let category = category.read();
    debug!("Build called from category: {}", &category.name);

    let author = &msg.author;
    debug!("Build called from author: {}#{}", &author.name, &author.discriminator);

    let author_voice_state = guild.voice_states.get(&author.id);
    debug!("Build called from voice_state: {:?}", &author_voice_state);

    if author_voice_state.is_none() {
        info!("Refusing to build for someone not in a voice channel");
        &msg.reply(&ctx.http, "You must be connected to a voice channel in order to build a new temp channel.")?;
        return Ok(());
    }

    let arg_channel_name = args.single_quoted::<String>()?;
    let channel_name = sanitize_channel_name(arg_channel_name.as_ref());

    channel.broadcast_typing(&ctx.http)?;
    debug!("Started typing");

    let created = guild.create_channel(&ctx.http, |c| {
        debug!("Channel name will be: {}", &channel_name);
        c.name(channel_name);

        debug!("Channel type will be: Voice");
        c.kind(ChannelType::Voice);

        debug!("Channel category will be: {}", &category_id);
        c.category(&category_id);

        debug!("Channel permissions will be cloned from the category");
        let mut permissions = category.permission_overwrites.clone();
        permissions.push(PermissionOverwrite{
            allow: Permissions::all(),
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(author.id)
        });
        c.permissions(permissions)
    })?;
    info!("Created channel #{}", &created.name);

    msg.reply(&ctx.http, format!("Temp channel <#{}> was built.", &created.id))?;
    debug!("Sent channel created message");

    guild.move_member(&ctx.http, &author.id, &created.id)?;
    debug!("Moved command caller to the created channel");

    Ok(())
}

/// Check whether an user left a channel and delete temp channels.
fn handle_voice_state_update(ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) -> result::Result<(), &'static str> {
    debug!("Handling voice_state_update");

    let guild_id = guild_id.ok_or("guild_id isn't available")?;
    let guild: PartialGuild = guild_id
        .to_partial_guild(&ctx.http).or(Err("Could not fetch guild data"))?;

    let old = old.ok_or("Old voice state isn't available")?;

    if let None = old.channel_id {
        return Err("User just connected to voice");
    }
    let old_channel_id = old.channel_id.unwrap();

    if let Some(new_channel_id) = new.channel_id {
        if old_channel_id == new_channel_id {
            return Err("Old channel is the same as the new one");
        }
    }

    let old_channel = old_channel_id
        .to_channel(&ctx.http).or(Err("Could not fetch channel data"))?
        .guild().unwrap();
    let old_channel = old_channel.read();

    let members: Vec<Member> = old_channel.members(&ctx.cache).or(Err("Failed to get members of the channel"))?;

    if members.len() != 0 {
        return Err("There still are members in the channel");
    }

    let mut bob_category_id : Option<ChannelId> = None;
    let bob_channel_name = env::var("BOB_CHANNEL_NAME").unwrap();
    let all_channels = guild.channels(&ctx.http).or(Err("Could not fetch all guild channels data"))?;
    for c in all_channels.values() {
        if c.name == bob_channel_name {
            let category_id = c.category_id;
            bob_category_id = Some(category_id.ok_or("category_id isn't available")?);
        }
    }

    bob_category_id.ok_or("Could not find the bob_category_id")?;

    if old_channel.category_id != bob_category_id {
        return Err("Channel isn't a temp channel");
    }

    old_channel.delete(&ctx.http).or(Err("Couldn't delete channel"))?;

    Ok(())
}