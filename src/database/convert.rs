//! This module contains the [BobFrom] trait, which enables fallible conversions between various types used by [diesel]
//! and [serenity].

use std::convert::{TryFrom};
use std::time::{Duration};
use serenity::model::prelude::{ChannelId, GuildId};
use crate::errors::{BobResult, BobCatch, ErrorKind};


pub trait BobFrom<T> {
    fn bobfrom(val: T) -> BobResult<Self> where Self: Sized;
}


impl BobFrom<i64> for ChannelId {
    fn bobfrom(val: i64) -> BobResult<Self> {
        let cid = u64::try_from(val)
            .bob_catch(ErrorKind::External, "i64 couldn't be converted into a u64?!")?;

        Ok(ChannelId(cid))
    }
}

impl BobFrom<ChannelId> for i64 {
    fn bobfrom(val: ChannelId) -> BobResult<Self> {
        let cid = i64::try_from(val.0)
            .bob_catch(ErrorKind::Developer, "ChannelId is larger than a i64")?;

        Ok(cid)
    }
}


impl BobFrom<i64> for GuildId {
    fn bobfrom(val: i64) -> BobResult<Self> {
        let cid = u64::try_from(val)
            .bob_catch(ErrorKind::Developer, "i64 couldn't be converted into a u64?!")?;

        Ok(GuildId(cid))
    }
}

impl BobFrom<GuildId> for i64 {
    fn bobfrom(val: GuildId) -> BobResult<Self> {
        let gid = i64::try_from(val.0)
            .bob_catch(ErrorKind::Developer, "GuildId is larger than a i64")?;

        Ok(gid)
    }
}


impl BobFrom<i32> for Duration {
    fn bobfrom(val: i32) -> BobResult<Self> {
        let time = u64::try_from(val)
            .bob_catch(ErrorKind::Developer, "i32 couldn't be converted into a u64?!")?;

        Ok(Duration::from_secs(time))
    }
}

impl BobFrom<Duration> for i32 {
    fn bobfrom(val: Duration) -> BobResult<Self> {
        let time = i32::try_from(val.as_secs())
            .bob_catch(ErrorKind::Developer, "Duration is larger than a i32")?;

        Ok(time)
    }
}