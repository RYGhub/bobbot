use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;


#[check]
#[name = "BobHasCategory"]
pub async fn check_bob_has_category(ctx: &Context, msg: &Message, _args: &mut Args) -> CheckResult {
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
    if channel.category_id.is_none() {
        return CheckResult::new_user("This channel isn't inside a category.");
    }

    let category = channel.category_id.unwrap().to_channel(&ctx.http).await;
    if category.is_err() {
        return CheckResult::new_log("Could not fetch bot category info from the Discord API.");
    }

    CheckResult::Success
}