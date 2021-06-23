//! This module contains traits which extend [serenity::model] objects.
//!
//! All extension methods are prefixed with `ext_`.

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::interactions::*;
use serenity::cache::Cache;
use serenity::http::Http;
use std::convert::{TryFrom};
use std::collections::{HashMap};
use async_trait::async_trait;
use crate::errors::*;


/// Trait which extends [Message].
#[async_trait]
pub trait MessageExtension {
    async fn ext_guild_id(&self) -> BobResult<GuildId>;
    async fn ext_reply(&self, http: &Http, content: String) -> BobResult<Message>;
}

#[async_trait]
impl MessageExtension for Message {
    async fn ext_guild_id(&self) -> BobResult<GuildId> {
        self.guild_id
            .ok_or_else(|| BobError::from_msg(ErrorKind::Developer, "Message does not have a guild_id"))
    }

    async fn ext_reply(&self, http: &Http, content: String) -> BobResult<Message> {
        self.reply(&http, content)
            .await.bob_catch(ErrorKind::Admin, "Couldn't reply to message")
    }
}


/// Trait which extends [GuildId].
#[async_trait]
pub trait GuildIdExtension {
    async fn ext_partial_guild(self, http: &Http) -> BobResult<PartialGuild>;
}

#[async_trait]
impl GuildIdExtension for GuildId {
    async fn ext_partial_guild(self, http: &Http) -> BobResult<PartialGuild> {
        self
            .to_partial_guild(&http)
            .await
            .bob_catch(ErrorKind::External, "Couldn't get Guild")
    }
}


/// Trait which extends [PartialGuild] and [Guild].
#[async_trait]
pub trait PartialGuildExtension {
    async fn ext_member(&self, http: &Http, user_id: UserId) -> BobResult<Member>;
}

#[async_trait]
impl PartialGuildExtension for PartialGuild {
    async fn ext_member(&self, http: &Http, user_id: UserId) -> BobResult<Member> {
        self.member(&http, user_id)
            .await
            .bob_catch(ErrorKind::Admin, "Couldn't get information about a server member")
    }
}

#[async_trait]
impl PartialGuildExtension for Guild {
    async fn ext_member(&self, http: &Http, user_id: UserId) -> BobResult<Member> {
        self.member(&http, user_id)
            .await
            .bob_catch(ErrorKind::Admin, "Couldn't get information about a server member")
    }
}


/// Trait which extends [ChannelId].
#[async_trait]
pub trait ChannelIdExtension {
    async fn ext_guild_channel(self, http: &Http) -> BobResult<GuildChannel>;
}

#[async_trait]
impl ChannelIdExtension for ChannelId {
    async fn ext_guild_channel(self, http: &Http) -> BobResult<GuildChannel> {
        self
            .to_channel(&http)
            .await
            .bob_catch(ErrorKind::External, "Couldn't retrieve channel info")?
            .guild()
            .bob_catch(ErrorKind::Developer, "Channel isn't a GuildChannel")
    }
}


/// Trait which extends [GuildChannel].
#[async_trait]
pub trait GuildChannelExtension {
    async fn ext_category(&self, http: &Http) -> BobResult<Option<ChannelCategory>>;
    async fn ext_bitrate(&self) -> BobResult<u32>;
    async fn ext_user_limit(&self) -> BobResult<Option<u32>>;
    async fn ext_members(&self, cache: &Cache) -> BobResult<Vec<Member>>;
    async fn ext_send_message(&self, http: &Http, content: String) -> BobResult<Message>;
}

#[async_trait]
impl GuildChannelExtension for GuildChannel {
    async fn ext_category(&self, http: &Http) -> BobResult<Option<ChannelCategory>> {
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

    async fn ext_bitrate(&self) -> BobResult<u32> {
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

    async fn ext_user_limit(&self) -> BobResult<Option<u32>> {
        match &self.user_limit {
            Some(user_limit) => Ok(Some(
                u32::try_from(user_limit.clone())
                    .bob_catch(ErrorKind::Developer, "User limit was somehow larger than a u32")?
            )),
            None => Ok(None),
        }
    }

    async fn ext_members(&self, cache: &Cache) -> BobResult<Vec<Member>> {
        self.members(&cache)
            .await
            .bob_catch(ErrorKind::External, "Could not fetch channel members")
    }

    async fn ext_send_message(&self, http: &Http, content: String) -> BobResult<Message> {
        self.send_message(&http, |m| m
            .content(content)
        ).await.bob_catch(ErrorKind::Admin, "Couldn't send message")
    }
}


/// Trait which extends [ApplicationCommandInteractionData].
pub trait ApplicationCommandInteractionDataExtension {
    fn option_hashmap(self) -> HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>>;
}

impl ApplicationCommandInteractionDataExtension for ApplicationCommandInteractionData {
    fn option_hashmap(self) -> HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>> {
        self.options
            .into_iter()
            .map(|option|
                (option.name, option.resolved)
            )
            .collect()
    }
}


/// Trait which extends the [HashMap] obtained by [ApplicationCommandInteractionDataExtension.option_hashmap].
pub trait ApplicationCommandInteractionDataHashmapExtension {
    fn req_string(&self, name: &'static str) -> BobResult<String>;
    fn req_integer(&self, name: &'static str) -> BobResult<i64>;
    fn req_boolean(&self, name: &'static str) -> BobResult<bool>;
    fn req_user(&self, name: &'static str) -> BobResult<User>;
    fn req_channel(&self, name: &'static str) -> BobResult<PartialChannel>;
    fn req_role(&self, name: &'static str) -> BobResult<Role>;
    fn opt_string(&self, name: &'static str) -> BobResult<Option<String>>;
    fn opt_integer(&self, name: &'static str) -> BobResult<Option<i64>>;
    fn opt_boolean(&self, name: &'static str) -> BobResult<Option<bool>>;
    fn opt_user(&self, name: &'static str) -> BobResult<Option<User>>;
    fn opt_channel(&self, name: &'static str) -> BobResult<Option<PartialChannel>>;
    fn opt_role(&self, name: &'static str) -> BobResult<Option<Role>>;
}


/// Retrieve a required argument from an [HashMap] obtained by [ApplicationCommandInteractionDataExtension.option_hashmap].
fn application_command_interaction_data_hashmap_extension_get_required_arg(
    hashmap: &HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>>,
    name: &str
)
    -> BobResult<ApplicationCommandInteractionDataOptionValue>
{
    let arg = hashmap.get(name)
        .bob_catch(ErrorKind::User, "Missing argument (in hashmap)")?;

    match arg {
        Some(v) => Ok(v.to_owned()),
        None => Err(BobError::from_msg(ErrorKind::User, "Missing argument (option is None)"))
    }
}

/// Retrieve an optional argument from an [HashMap] obtained by [ApplicationCommandInteractionDataExtension.option_hashmap].
fn application_command_interaction_data_hashmap_extension_get_optional_arg(
    hashmap: &HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>>,
    name: &str
)
    -> Option<ApplicationCommandInteractionDataOptionValue>
{
    let arg = hashmap.get(name);

    match arg {
        Some(o) => match o {
            Some(v) => Some(v.to_owned()),
            None => None
        }
        None => None
    }
}

macro_rules! arg_required {
    ($fn_name:ident, $type:ty, $kind:path) => {
        fn $fn_name(&self, name: &str) -> BobResult<$type> {
            match application_command_interaction_data_hashmap_extension_get_required_arg(&self, &name)? {
                $kind(a, ..) => Ok(a),
                _     => Err(BobError::from_msg(ErrorKind::Developer, "Argument is of an invalid type"))
            }
        }
    }
}

macro_rules! arg_optional {
    ($fn_name:ident, $type:ty, $kind:path) => {
        fn $fn_name(&self, name: &str) -> BobResult<Option<$type>> {
            match application_command_interaction_data_hashmap_extension_get_optional_arg(&self, &name) {
                Some(v) => match v {
                    $kind(a, ..) => Ok(Some(a)),
                    _     => Err(BobError::from_msg(ErrorKind::Developer, "Argument is of an invalid type"))
                }
                None => Ok(None),
            }
        }
    }
}

macro_rules! arg {
    ($fn_req:ident, $fn_opt:ident, $type:ty, $kind:path) => {
        arg_required!($fn_req, $type, $kind);
        arg_optional!($fn_opt, $type, $kind);
    }
}

impl ApplicationCommandInteractionDataHashmapExtension for HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>> {
    arg!(req_string,  opt_string,   String,         ApplicationCommandInteractionDataOptionValue::String);
    arg!(req_integer, opt_integer,  i64,            ApplicationCommandInteractionDataOptionValue::Integer);
    arg!(req_boolean, opt_boolean,  bool,           ApplicationCommandInteractionDataOptionValue::Boolean);
    arg!(req_user,    opt_user,     User,           ApplicationCommandInteractionDataOptionValue::User);
    arg!(req_channel, opt_channel,  PartialChannel, ApplicationCommandInteractionDataOptionValue::Channel);
    arg!(req_role,    opt_role,     Role,           ApplicationCommandInteractionDataOptionValue::Role);
}


#[async_trait]
pub trait InteractionExtension {
    async fn pong(&self, http: &Http) -> BobResult<()>;
}

#[async_trait]
impl InteractionExtension for Interaction {
    async fn pong(&self, http: &Http) -> BobResult<()> {
        self.create_interaction_response(&http, |r| r
            .kind(InteractionResponseType::Pong)
        ).await.bob_catch(ErrorKind::Host, "Couldn't reply to ping interaction")
    }
}