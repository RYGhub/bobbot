use serenity::model::prelude::{GuildId, PartialGuild, ChannelId};
use crate::basics::result::{BobResult, result_error};
use serenity::http::Http;
use serenity::model::voice::VoiceState;

pub async fn get_partial_guild(http: &Http, guild_id: GuildId) -> BobResult<PartialGuild> {
    guild_id
        .to_partial_guild(&http)
        .await
        .map_err(|e| result_error(e, "Could not fetch guild data"))
}


