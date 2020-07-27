use std::env;
use serenity::Client;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::StandardFramework;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

struct BobHandler;

#[group]
#[commands(build)]
struct Bob;


fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    env::var("BOB_CHANNEL_NAME").expect("Missing BOB_CHANNEL_NAME");

    let mut client = Client::new(&token, BobHandler).expect("Error creating Discord client");

    client.with_framework(
        StandardFramework::new().configure(
            |c| c
                .prefix("!")
        )
        .group(&BOB_GROUP)
        .on_dispatch_error(error)
    );

    client.start_autosharded().expect("Error starting Discord client");
}


impl EventHandler for BobHandler {
    fn ready(&self, _context: Context, ready: Ready) {
        println!("{} is ready!", ready.user.name);
    }

    // IntelliJ Rust inspection is broken
    // https://github.com/intellij-rust/intellij-rust/issues/1191
    // noinspection RsTraitImplementation
    fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, _new: VoiceState) {
        let guild: PartialGuild = guild_id
            .expect("guild_id wasn't available")
            .to_partial_guild(&ctx.http).expect("Could not fetch guild data");

        if let None = old {
            return;
        }
        let old = old.unwrap();

        if let None = old.channel_id {
            return;
        }
        let channel = old.channel_id.unwrap()
            .to_channel(&ctx.http).expect("Could not fetch channel data.")
            .guild().unwrap();
        let channel = channel.read();

        let members: Vec<Member> = channel.members(&ctx.cache).expect("Failed to get members of the channel.");

        if members.len() != 0 {
            return;
        }

        let mut bob_category_id : Option<ChannelId> = None;
        let bob_channel_name = env::var("BOB_CHANNEL_NAME").unwrap();
        let all_channels = guild.channels(&ctx.http).expect("Could not fetch channels data.");
        for c in all_channels.values() {
            if c.name == bob_channel_name {
                bob_category_id = Some(c.category_id.expect("Category does not have a channel_id."));
            }
        }
        if let None = bob_category_id {
            return;
        }

        if channel.category_id != bob_category_id {
            return;
        }

        channel.delete(&ctx.http).expect("Failed to delete channel.");
    }
}


fn error(ctx: &mut Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::OnlyForGuilds => {
            let _ = msg.reply(&ctx.http, "This command can only be sent in a guild.");
        }
        _ => {
            let _ = msg.reply(&ctx.http, "Unmatched error occoured.");
        }
    }
}


#[command]
#[only_in(guilds)]
fn build(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = &msg
        .guild(&ctx.cache).expect("Call passed only_in(guilds) check but didn't have a guild.");
    let channel = &msg
        .channel(&ctx.cache).expect("Call passed only_in(guilds) check but didn't have a channel_id.")
        .guild().expect("Call passed only_in(guilds) check but wasn't a GuildChannel.");

    let arg_channel_name = args.single_quoted::<String>()?;

    channel.read().broadcast_typing(&ctx.http)?;

    let created = guild.read().create_channel(&ctx.http, |c| {
        let c = c.name(arg_channel_name);
        let c = c.kind(ChannelType::Voice);
        match channel.read().category_id {
            Some(category_id) => c.category(category_id),
            None => c
        };
        c
    }
    )?;
    msg.reply(&ctx.http, format!("Channel <#{}> was built.", &created.id))?;

    Ok(())
}