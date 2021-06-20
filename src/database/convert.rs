use std::convert::{TryFrom};
use std::time::{Duration};
use serenity::model::prelude::{ChannelId, GuildId};
use crate::errors::{bot_error, BobResult};


pub trait BobFrom<T> {
    fn bobfrom(val: T) -> BobResult<Self>;
}


impl BobFrom<i64> for ChannelId {
    fn bobfrom(val: i64) -> BobResult<Self> {
        let cid = u64::try_from(val)
            .map_err(bot_error)?;

        Ok(ChannelId(cid))
    }
}

impl BobFrom<ChannelId> for i64 {
    fn bobfrom(val: ChannelId) -> BobResult<Self> {
        let cid = i64::try_from(val.0)
            .map_err(bot_error)?;

        Ok(cid)
    }
}


impl BobFrom<i64> for GuildId {
    fn bobfrom(val: i64) -> BobResult<Self> {
        let cid = u64::try_from(val)
            .map_err(bot_error)?;

        Ok(GuildId(cid))
    }
}

impl BobFrom<GuildId> for i64 {
    fn bobfrom(val: GuildId) -> BobResult<Self> {
        let gid = i64::try_from(val.0)
            .map_err(bot_error)?;

        Ok(gid)
    }
}


impl BobFrom<i32> for Duration {
    fn bobfrom(val: i32) -> BobResult<Self> {
        let time = u64::try_from(val)
            .map_err(bot_error)?;

        Ok(Duration::from_secs(time))
    }
}

impl BobFrom<Duration> for i32 {
    fn bobfrom(val: Duration) -> BobResult<Self> {
        let time = i32::try_from(val.as_secs())
            .map_err(bot_error)?;

        Ok(time)
    }
}