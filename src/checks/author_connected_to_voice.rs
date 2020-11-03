use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;


#[check]
#[name = "AuthorConnectedToVoice"]
pub async fn check_author_connected_to_voice(ctx: &Context, msg: &Message, _args: &mut Args) -> CheckResult {
    let guild = msg.guild(&ctx.cache).await;
    if guild.is_none() {
        return CheckResult::new_log("Could not fetch guild info from the Discord API.");
    }

    let guild = guild.unwrap();

    let author_voice_state = guild.voice_states.get(&msg.author.id);
    if author_voice_state.is_none() {
        return CheckResult::new_user("You must be connected to a voice channel in order to run this command.");
    }

    let author_voice_state = author_voice_state.unwrap();
    if author_voice_state.channel_id.is_none() {
        return CheckResult::new_user("You must be connected to a voice channel in order to run this command.");
    }

    CheckResult::Success
}