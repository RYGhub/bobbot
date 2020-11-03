use std::env;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;
use once_cell::sync::Lazy;


#[check]
#[name = "SentInBob"]
pub async fn check_sent_in_bob(ctx: &Context, msg: &Message, _args: &mut Args) -> CheckResult {
    static BOB_CHANNEL_NAME: Lazy<String> = Lazy::new(|| {env::var("BOB_CHANNEL_NAME").expect("Missing BOB_CHANNEL_NAME envvar.")});

    let channel = msg.channel(&ctx.cache).await;
    if channel.is_none() {
        return CheckResult::new_log("Could not fetch bot channel info from the Discord API.");
    }

    let channel = channel.unwrap();
    let channel = channel.guild();
    if channel.is_none() {
        return CheckResult::new_user("This channel isn't inside a server.");
    }

    let channel = channel.unwrap();
    if channel.name != *BOB_CHANNEL_NAME {
        return CheckResult::new_user(format!("This channel isn't named #{}, so commands won't run here.", &*BOB_CHANNEL_NAME));
    }

    CheckResult::Success
}