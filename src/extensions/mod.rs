//! This module contains traits which extend [serenity::model] objects.

use serenity::model::prelude::*;
use serenity::cache::{Cache};
use serenity::http::{Http};
use std::convert::{TryFrom};
use async_trait::async_trait;
use crate::errors::*;

#[async_trait]
pub trait BobMessage {
    async fn bob_guild_id(&self) -> BobResult<GuildId>;
}

#[async_trait]
pub trait BobGuildId {
    async fn bob_guild(self, cache: &Cache) -> BobResult<Guild>;
}

#[async_trait]
pub trait BobGuild {
    async fn bob_member(&self, http: &Http, user_id: UserId) -> BobResult<Member>;
}

#[async_trait]
pub trait BobChannelId {
    async fn bob_guild_channel(self, http: &Http) -> BobResult<GuildChannel>;
}

#[async_trait]
pub trait BobGuildChannel {
    async fn bob_category(&self, http: &Http) -> BobResult<Option<ChannelCategory>>;
    async fn bob_bitrate(&self) -> BobResult<u32>;
    async fn bob_user_limit(&self) -> BobResult<Option<u32>>;
    async fn bob_members(&self, cache: &Cache) -> BobResult<Vec<Member>>;
}

#[async_trait]
impl BobMessage for Message {
    async fn bob_guild_id(&self) -> BobResult<GuildId> {
        self.guild_id
            .ok_or_else(|| BobError::from_msg(ErrorKind::Developer, "Message does not have a guild_id"))
    }
}

#[async_trait]
impl BobGuildId for GuildId {
    async fn bob_guild(self, cache: &Cache) -> BobResult<Guild> {
        self
            .bob_guild(&cache)
            .await
            .bob_catch(ErrorKind::External, "Couldn't get Guild")
    }
}

#[async_trait]
impl BobGuild for Guild {
    async fn bob_member(&self, http: &Http, user_id: UserId) -> BobResult<Member> {
        self.member(&http, user_id)
            .await
            .bob_catch(ErrorKind::Admin, "Couldn't get information about a server member")
    }
}

#[async_trait]
impl BobChannelId for ChannelId {
    async fn bob_guild_channel(self, http: &Http) -> BobResult<GuildChannel> {
        self
            .to_channel(&http)
            .await
            .bob_catch(ErrorKind::External, "Couldn't retrieve channel info")?
            .guild()
            .bob_catch(ErrorKind::Developer, "Channel isn't a GuildChannel")
    }
}

#[async_trait]
impl BobGuildChannel for GuildChannel {
    async fn bob_category(&self, http: &Http) -> BobResult<Option<ChannelCategory>> {
        match &self.category_id {
            None => {
                Ok(None)
            },
            Some(category_id) => {
                let category = category_id
                    .to_channel(&http)
                    .await
                    .bob_catch(ErrorKind::External, "Couldn't retrieve channel info")?
                    .category()
                    .bob_catch(ErrorKind::Developer, "Channel isn't a ChannelCategory")?;

                Ok(Some(category))
            }
        }
    }

    async fn bob_bitrate(&self) -> BobResult<u32> {
        match &self.bitrate {
            Some(bitrate) => Ok(
                u32::try_from(bitrate.clone())
                    .bob_catch(ErrorKind::Developer, "Bitrate was somehow larger than a u32")?
            ),
            None => Err(
                BobError::from_msg(ErrorKind::Developer, "Channel does not have any bitrate (not a voice channel?)")
            ),
        }
    }

    async fn bob_user_limit(&self) -> BobResult<Option<u32>> {
        match &self.user_limit {
            Some(user_limit) => Ok(Some(
                u32::try_from(user_limit.clone())
                    .bob_catch(ErrorKind::Developer, "User limit was somehow larger than a u32")?
            )),
            None => Ok(None),
        }
    }

    async fn bob_members(&self, cache: &Cache) -> BobResult<Vec<Member>> {
        self.members(&cache)
            .await
            .bob_catch(ErrorKind::External, "Could not fetch channel members")
    }
}
