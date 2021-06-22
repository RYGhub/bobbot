//! This module contains traits which extend [serenity::model] objects.

use serenity::model::prelude::*;
use serenity::model::interactions::{Interaction};
use serenity::cache::{Cache};
use serenity::http::{Http};
use std::convert::{TryFrom};
use std::collections::{HashMap};
use async_trait::async_trait;
use crate::errors::*;
use serenity::model::prelude::ApplicationCommandInteractionDataOptionValue;

#[async_trait]
pub trait BobMessage {
    async fn bob_guild_id(&self) -> BobResult<GuildId>;
    async fn bob_reply(&self, http: &Http, content: String) -> BobResult<Message>;
}

#[async_trait]
impl BobMessage for Message {
    async fn bob_guild_id(&self) -> BobResult<GuildId> {
        self.guild_id
            .ok_or_else(|| BobError::from_msg(ErrorKind::Developer, "Message does not have a guild_id"))
    }

    async fn bob_reply(&self, http: &Http, content: String) -> BobResult<Message> {
        self.reply(&http, content)
            .await.bob_catch(ErrorKind::Admin, "Couldn't reply to message")
    }
}


#[async_trait]
pub trait BobGuildId {
    async fn bob_partial_guild(self, http: &Http) -> BobResult<PartialGuild>;
}

#[async_trait]
impl BobGuildId for GuildId {
    async fn bob_partial_guild(self, http: &Http) -> BobResult<PartialGuild> {
        self
            .to_partial_guild(&http)
            .await
            .bob_catch(ErrorKind::External, "Couldn't get Guild")
    }
}


#[async_trait]
pub trait BobGuild {
    async fn bob_member(&self, http: &Http, user_id: UserId) -> BobResult<Member>;
}

#[async_trait]
impl BobGuild for PartialGuild {
    async fn bob_member(&self, http: &Http, user_id: UserId) -> BobResult<Member> {
        self.member(&http, user_id)
            .await
            .bob_catch(ErrorKind::Admin, "Couldn't get information about a server member")
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
pub trait BobChannelId {
    async fn bob_guild_channel(self, http: &Http) -> BobResult<GuildChannel>;
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
pub trait BobGuildChannel {
    async fn bob_category(&self, http: &Http) -> BobResult<Option<ChannelCategory>>;
    async fn bob_bitrate(&self) -> BobResult<u32>;
    async fn bob_user_limit(&self) -> BobResult<Option<u32>>;
    async fn bob_members(&self, cache: &Cache) -> BobResult<Vec<Member>>;
    async fn bob_send_message(&self, http: &Http, content: String) -> BobResult<Message>;
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

    async fn bob_send_message(&self, http: &Http, content: String) -> BobResult<Message> {
        self.send_message(&http, |m| m
            .content(content)
        ).await.bob_catch(ErrorKind::Admin, "Couldn't send message")
    }
}


pub trait BobApplicationCommandInteractionData {
    fn option_hashmap(self) -> HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>>;
}

impl BobApplicationCommandInteractionData for ApplicationCommandInteractionData {
    fn option_hashmap(self) -> HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>> {
        self.options
            .into_iter()
            .map(|option|
                (option.name, option.resolved)
            )
            .collect()
    }
}


pub trait BobApplicationCommandInteractionDataHashmap {
    fn arg_req_string(&self, name: &'static str) -> BobResult<String>;
    fn arg_req_integer(&self, name: &'static str) -> BobResult<i64>;
    fn arg_req_boolean(&self, name: &'static str) -> BobResult<bool>;
    fn arg_req_user(&self, name: &'static str) -> BobResult<User>;
    fn arg_req_channel(&self, name: &'static str) -> BobResult<PartialChannel>;
    fn arg_req_role(&self, name: &'static str) -> BobResult<Role>;
    fn arg_opt_string(&self, name: &'static str) -> BobResult<Option<String>>;
    fn arg_opt_integer(&self, name: &'static str) -> BobResult<Option<i64>>;
    fn arg_opt_boolean(&self, name: &'static str) -> BobResult<Option<bool>>;
    fn arg_opt_user(&self, name: &'static str) -> BobResult<Option<User>>;
    fn arg_opt_channel(&self, name: &'static str) -> BobResult<Option<PartialChannel>>;
    fn arg_opt_role(&self, name: &'static str) -> BobResult<Option<Role>>;
}

fn get_required_arg_from_hashmap(hashmap: &HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>>, name: &str) -> BobResult<ApplicationCommandInteractionDataOptionValue> {
    let arg = hashmap.get(name)
        .bob_catch(ErrorKind::User, "Missing argument (in hashmap)")?;

    match arg {
        Some(v) => Ok(v.to_owned()),
        None => Err(BobError::from_msg(ErrorKind::User, "Missing argument (option is None)"))
    }
}

fn get_optional_arg_from_hashmap(hashmap: &HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>>, name: &str) -> Option<ApplicationCommandInteractionDataOptionValue> {
    let arg = hashmap.get(name);

    match arg {
        Some(o) => match o {
            Some(v) => Some(v.to_owned()),
            None => None
        }
        None => None
    }
}

macro_rules! args_required {
    ( $n:ident: $t:ty = $v:path ) => {
        fn $n(&self, name: &str) -> BobResult<$t> {
            match get_required_arg_from_hashmap(&self, &name)? {
                $v(s) => Ok(s),
                _ => Err(BobError::from_msg(ErrorKind::Developer, "Argument is of an invalid type"))
            }
        }
    }
}

macro_rules! args_optional {
    ( $n:ident: $t:ty = $v:path ) => {
        fn $n(&self, name: &str) -> BobResult<Option<$t>> {
            match get_optional_arg_from_hashmap(&self, &name) {
                Some(v) => match v {
                    $v(s) => Ok(Some(s)),
                    _ => Err(BobError::from_msg(ErrorKind::Developer, "Argument is of an invalid type"))
                }
                None => Ok(None),
            }
        }
    }
}

impl BobApplicationCommandInteractionDataHashmap for HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>> {
    args_required!(arg_req_string: String = ApplicationCommandInteractionDataOptionValue::String);
    args_required!(arg_req_integer: i64 = ApplicationCommandInteractionDataOptionValue::Integer);
    args_required!(arg_req_boolean: bool = ApplicationCommandInteractionDataOptionValue::Boolean);
    args_required!(arg_req_channel: PartialChannel = ApplicationCommandInteractionDataOptionValue::Channel);
    args_required!(arg_req_role: Role = ApplicationCommandInteractionDataOptionValue::Role);

    fn arg_req_user(&self, name: &str) -> BobResult<User> {
        match get_required_arg_from_hashmap(&self, &name)? {
            ApplicationCommandInteractionDataOptionValue::User(s, _) => Ok(s),
            _ => Err(BobError::from_msg(ErrorKind::Developer, "Argument is of an invalid type"))
        }
    }

    args_optional!(arg_opt_string: String = ApplicationCommandInteractionDataOptionValue::String);
    args_optional!(arg_opt_integer: i64 = ApplicationCommandInteractionDataOptionValue::Integer);
    args_optional!(arg_opt_boolean: bool = ApplicationCommandInteractionDataOptionValue::Boolean);
    args_optional!(arg_opt_channel: PartialChannel = ApplicationCommandInteractionDataOptionValue::Channel);
    args_optional!(arg_opt_role: Role = ApplicationCommandInteractionDataOptionValue::Role);

    fn arg_opt_user(&self, name: &str) -> BobResult<Option<User>> {
        match get_optional_arg_from_hashmap(&self, &name) {
            Some(v) => match v {
                ApplicationCommandInteractionDataOptionValue::User(s, _) => Ok(Some(s)),
                _ => Err(BobError::from_msg(ErrorKind::Developer, "Argument is of an invalid type"))
            }
            None => Ok(None),
        }
    }
}


#[async_trait]
pub trait BobInteraction {
    async fn pong(&self, http: &Http) -> BobResult<()>;
}

#[async_trait]
impl BobInteraction for Interaction {
    async fn pong(&self, http: &Http) -> BobResult<()> {
        self.create_interaction_response(&http, |r| r
            .kind(InteractionResponseType::Pong)
        ).await.bob_catch(ErrorKind::Host, "Couldn't reply to ping interaction")
    }
}