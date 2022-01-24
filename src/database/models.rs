//! This module contains the database ORM models.

use std::env::{var};
use std::time::{Duration};
use diesel::prelude::*;
use serenity::model::prelude::{ChannelId, GuildId, GuildChannel, ChannelType};
use serenity::model::channel::{PermissionOverwrite};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::errors::{BobResult, BobCatch, ErrorKind, BobError};
use crate::database::schema::{command_channels, deletion_times, channels_created, presets};
use crate::database::convert::{BobFrom};


/// Create a new database [PgConnection].
///
/// # Panics
///
/// If the `DATABASE_URL` environment variable is not set, or if the database connection fails.
pub fn connect() -> PgConnection {
    let database_url = var("DATABASE_URL")
        .expect("DATABASE_URL is not set");

    PgConnection::establish(&database_url)
        .expect("Couldn't connect to database")
}


pub enum DatabaseAction<T> {
    Created(T),
    Updated(T),
    Deleted,
    None,
}


#[derive(Queryable, Insertable)]
#[table_name="command_channels"]
pub struct CommandChannel {
    pub guild_id: i64,
    pub channel_id: i64,
}

impl CommandChannel {
    /// Get the raw [CommandChannel] struct for the given guild id.
    fn get_raw(gid: i64) -> BobResult<Option<CommandChannel>> {
        use crate::database::schema::command_channels::dsl::*;

        let mut results: Vec<CommandChannel> = command_channels
            .filter(guild_id.eq(gid))
            .limit(1)
            .load::<CommandChannel>(&connect())
            .bob_catch(ErrorKind::External, "Couldn't retrieve Command Channel information from the database.")?;

        match results.len() {
            0 => Ok(None),
            _ => Ok(Some(results.swap_remove(0)))
        }
    }

    /// Set the raw [CommandChannel] struct for the given guild id.
    fn set_raw(gid: i64, cid: i64) -> BobResult<DatabaseAction<CommandChannel>> {
        use crate::database::schema::command_channels::dsl::*;

        if let Some(cc) = CommandChannel::get_raw(gid)? {
            let result = diesel::update(command_channels.find(cc.guild_id))
                .set(channel_id.eq(cid))
                .get_result::<CommandChannel>(&connect())
                .bob_catch(ErrorKind::External, "Couldn't edit Command Channel information in the database.")?;

            return Ok(DatabaseAction::Updated(result));
        }
        else {
            let cc = CommandChannel {
                guild_id: gid,
                channel_id: cid,
            };

            let result = diesel::insert_into(command_channels)
                .values(&cc)
                .get_result::<CommandChannel>(&connect())
                .bob_catch(ErrorKind::External, "Couldn't edit Command Channel information in the database.")?;

            return Ok(DatabaseAction::Created(result));
        }
    }

    /// Unset the raw [CommandChannel] struct for the given guild id.
    fn unset_raw(gid: i64) -> BobResult<DatabaseAction<CommandChannel>> {
        use crate::database::schema::command_channels::dsl::*;

        match CommandChannel::get_raw(gid)? {
            None => {
                Ok(DatabaseAction::None)
            },
            Some(cc) => {
                diesel::delete(command_channels.find(cc.guild_id)).execute(&connect())
                    .bob_catch(ErrorKind::Host, "Couldn't unset Command Channel in the database.")?;
                Ok(DatabaseAction::Deleted)
            },
        }
    }
}

pub trait WithCommandChannel {
    /// Get the command [ChannelId] for the given [GuildId].
    fn get_command_channel(&self) -> BobResult<Option<ChannelId>>;

    /// Set the command [ChannelId] for the given [GuildId].
    fn set_command_channel(&self, cid: ChannelId) -> BobResult<DatabaseAction<CommandChannel>>;

    /// Unset the command channel for the given [GuildId].
    fn unset_command_channel(&self) -> BobResult<DatabaseAction<CommandChannel>>;

    /// Either set or unset the command channel for the given [GuildId].
    fn edit_command_channel(&self, cid: Option<ChannelId>) -> BobResult<DatabaseAction<CommandChannel>>;
}

impl WithCommandChannel for GuildId {
    fn get_command_channel(&self) -> BobResult<Option<ChannelId>> {
        let gid = i64::bobfrom(self.clone())?;

        match CommandChannel::get_raw(gid)? {
            None => Ok(None),
            Some(v) => Ok(Some(ChannelId::bobfrom(v.channel_id)?))
        }
    }

    fn set_command_channel(&self, cid: ChannelId) -> BobResult<DatabaseAction<CommandChannel>> {
        let gid = i64::bobfrom(self.clone())?;
        let cid = i64::bobfrom(cid)?;

        CommandChannel::set_raw(gid, cid)
    }

    fn unset_command_channel(&self) -> BobResult<DatabaseAction<CommandChannel>> {
        let gid = i64::bobfrom(self.clone())?;

        CommandChannel::unset_raw(gid)
    }

    fn edit_command_channel(&self, cid: Option<ChannelId>) -> BobResult<DatabaseAction<CommandChannel>> {
        match cid {
            Some(cid) => self.set_command_channel(cid),
            None => self.unset_command_channel(),
        }
    }
}



#[derive(Queryable, Insertable)]
#[table_name="deletion_times"]
pub struct DeletionTime {
    pub guild_id: i64,
    pub deletion_time: i32,
}

impl DeletionTime {
    /// Get the raw [DeletionTime] struct for the given guild id.
    fn get_raw(gid: i64) -> BobResult<Option<DeletionTime>> {
        use crate::database::schema::deletion_times::dsl::*;

        let mut results: Vec<DeletionTime> = deletion_times
            .filter(guild_id.eq(gid))
            .limit(1)
            .load::<DeletionTime>(&connect())
            .bob_catch(ErrorKind::External, "Couldn't retrieve Deletion Time information from the database.")?;

        match results.len() {
            0 => Ok(None),
            _ => Ok(Some(results.swap_remove(0)))
        }
    }

    /// Set the raw [DeletionTime] struct for the given guild id.
    fn set_raw(gid: i64, time: i32) -> BobResult<DatabaseAction<DeletionTime>> {
        use crate::database::schema::deletion_times::dsl::*;

        if let Some(dt) = DeletionTime::get_raw(gid)? {
            let result = diesel::update(deletion_times.find(dt.guild_id))
                .set(deletion_time.eq(time))
                .get_result::<DeletionTime>(&connect())
                .bob_catch(ErrorKind::External, "Couldn't edit Deletion Time information in the database.")?;

            return Ok(DatabaseAction::Updated(result));
        }
        else {
            let dt = DeletionTime {
                guild_id: gid,
                deletion_time: time,
            };

            let result = diesel::insert_into(deletion_times)
                .values(&dt)
                .get_result::<DeletionTime>(&connect())
                .bob_catch(ErrorKind::External, "Couldn't edit Deletion Time information in the database.")?;

            return Ok(DatabaseAction::Created(result));
        }
    }

    /// Unset the raw [DeletionTime] struct for the given guild id.
    fn unset_raw(gid: i64) -> BobResult<DatabaseAction<DeletionTime>> {
        use crate::database::schema::deletion_times::dsl::*;

        match DeletionTime::get_raw(gid)? {
            None => {
                Ok(DatabaseAction::None)
            },
            Some(dt) => {
                diesel::delete(deletion_times.find(dt.guild_id)).execute(&connect())
                    .bob_catch(ErrorKind::External, "Couldn't delete Deletion Time information in the database.")?;
                Ok(DatabaseAction::Deleted)
            },
        }
    }
}

pub trait WithDeletionTime {
    /// Get the deletion time for the given [GuildId].
    fn get_deletion_time(&self) -> BobResult<Option<Duration>>;

    /// Set the deletion time for the given [GuildId].
    fn set_deletion_time(&self, time: Duration) -> BobResult<DatabaseAction<DeletionTime>>;

    /// Unset the deletion time for the given [GuildId].
    fn unset_deletion_time(&self) -> BobResult<DatabaseAction<DeletionTime>>;

    /// Either set or unset the deletion time for the given [GuildId].
    fn edit_deletion_time(&self, cid: Option<Duration>) -> BobResult<DatabaseAction<DeletionTime>>;
}

impl WithDeletionTime for GuildId {
    fn get_deletion_time(&self) -> BobResult<Option<Duration>> {
        let gid = i64::bobfrom(self.clone())?;

        match DeletionTime::get_raw(gid)? {
            None => Ok(None),
            Some(v) => Ok(Some(Duration::bobfrom(v.deletion_time)?))
        }
    }

    fn set_deletion_time(&self, time: Duration) -> BobResult<DatabaseAction<DeletionTime>> {
        let gid = i64::bobfrom(self.clone())?;
        let time = i32::bobfrom(time)?;

        DeletionTime::set_raw(gid, time)
    }

    fn unset_deletion_time(&self) -> BobResult<DatabaseAction<DeletionTime>> {
        let gid = i64::bobfrom(self.clone())?;

        DeletionTime::unset_raw(gid)
    }

    fn edit_deletion_time(&self, duration: Option<Duration>) -> BobResult<DatabaseAction<DeletionTime>> {
        match duration {
            Some(duration) => self.set_deletion_time(duration),
            None => self.unset_deletion_time(),
        }
    }
}


#[derive(Queryable, Insertable)]
#[table_name="channels_created"]
pub struct CreatedChannel {
    pub guild_id: i64,
    pub channel_id: i64,
}

impl CreatedChannel {
    fn get_all_raw(gid: i64) -> BobResult<Vec<CreatedChannel>> {
        use crate::database::schema::channels_created::dsl::*;

        channels_created
            .filter(guild_id.eq(gid))
            .load::<CreatedChannel>(&connect())
            .bob_catch(ErrorKind::External, "Couldn't retrieve Created Channels from the database.")
    }

    fn get_all_less_raw(gid: i64) -> BobResult<Vec<i64>> {
        Ok(CreatedChannel::get_all_raw(gid)?.iter().map(|v| v.channel_id).collect())
    }

    fn get_raw(gid: i64, cid: i64) -> BobResult<Option<CreatedChannel>> {
        use crate::database::schema::channels_created::dsl::*;

        let mut results =
            channels_created
                .filter(guild_id.eq(gid).and(channel_id.eq(cid)))
                .limit(1)
                .load::<CreatedChannel>(&connect())
                .bob_catch(ErrorKind::External, "Couldn't retrieve Created Channels from the database.")?;

        match results.len() {
            0 => Ok(None),
            _ => Ok(Some(results.swap_remove(0)))
        }
    }

    fn put_raw(gid: i64, cid: i64) -> BobResult<CreatedChannel> {
        use crate::database::schema::channels_created::dsl::*;

        match CreatedChannel::get_raw(gid, cid)? {
            Some(v) => {
                Ok(v)
            },
            None => {
                let cc = CreatedChannel {guild_id: gid, channel_id: cid};

                diesel::insert_into(channels_created)
                    .values(&cc)
                    .get_result::<CreatedChannel>(&connect())
                    .bob_catch(ErrorKind::External, "Couldn't add a new Created Channel into the database.")
            }
        }
    }
}

pub trait MayHaveBeenCreatedByBob {
    fn was_created_by_bob(&self) -> BobResult<bool>;
    fn mark_as_created_by_bob(&self) -> BobResult<CreatedChannel>;
}

impl MayHaveBeenCreatedByBob for GuildChannel {
    fn was_created_by_bob(&self) -> BobResult<bool> {
        match CreatedChannel::get_raw(i64::bobfrom(self.guild_id)?, i64::bobfrom(self.id)?)? {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    fn mark_as_created_by_bob(&self) -> BobResult<CreatedChannel> {
        debug!("Marking {} as created by Bob", &self.id);
        CreatedChannel::put_raw(i64::bobfrom(self.guild_id)?, i64::bobfrom(self.id)?)
    }
}


#[derive(Queryable, Insertable)]
#[table_name="presets"]
pub struct Preset {
    pub guild_id: i64,
    pub preset_name: String,
    pub preset_data: serde_json::Value,
}


impl Preset {
    fn get_all_raw(gid: i64) -> BobResult<Vec<Preset>> {
        use crate::database::schema::presets::dsl::*;

        presets
            .filter(guild_id.eq(gid))
            .load::<Preset>(&connect())
            .bob_catch(ErrorKind::External, "Couldn't retrieve Presets from the database.")
    }

    fn get_raw(gid: i64, name: &str) -> BobResult<Option<Preset>> {
        use crate::database::schema::presets::dsl::*;

        let mut results =
            presets
                .filter(guild_id.eq(gid).and(preset_name.eq(name)))
                .limit(1)
                .load::<Preset>(&connect())
                .bob_catch(ErrorKind::External, "Couldn't retrieve Presets from the database.")?;

        match results.len() {
            0 => Ok(None),
            _ => Ok(Some(results.swap_remove(0)))
        }
    }

    fn save_raw(gid: i64, name: String, data: PresetData, overwrite: bool) -> BobResult<DatabaseAction<Preset>> {
        use crate::database::schema::presets::dsl::*;

        let data = serde_json::to_value::<PresetData>(data)
            .bob_catch(ErrorKind::Developer, "Couldn't serialize PresetData.")?;

        match Preset::get_raw(gid, &name)? {
            Some(pr) => {
                if !overwrite {
                    return Ok(DatabaseAction::None)
                }

                let result = diesel::update(presets.find((pr.guild_id, pr.preset_name)))
                    .set(preset_data.eq(data))
                    .get_result::<Preset>(&connect())
                    .bob_catch(ErrorKind::External, "Couldn't edit Preset in the database.")?;

                Ok(DatabaseAction::Updated(result))
            },
            None => {
                let cc = Preset {
                    guild_id: gid,
                    preset_name: name,
                    preset_data: data,
                };

                diesel::insert_into(presets)
                    .values(&cc)
                    .get_result::<Preset>(&connect())
                    .bob_catch(ErrorKind::External, "Couldn't add a new Preset into the database.")?;

                Ok(DatabaseAction::Created(cc))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetData {
    pub bitrate: u64,
    pub user_limit: Option<u64>,
    pub permissions: Vec<PermissionOverwrite>
}

pub trait CanGetPresetData {
    fn get_preset_data(&self, name: &str) -> BobResult<Option<PresetData>>;
}

impl CanGetPresetData for GuildId {
    fn get_preset_data(&self, name: &str) -> BobResult<Option<PresetData>> {
        let preset = Preset::get_raw(self.0 as i64, name)?;
        match preset {
            Some (preset) => Ok(Some(
                serde_json::from_value::<PresetData>(preset.preset_data)
                    .bob_catch(ErrorKind::Developer, "Couldn't deserialize PresetData")?
            )),
            None => Ok(None),
        }
    }
}

pub trait IntoPresetData {
    fn preset_data(self) -> BobResult<PresetData>;
    fn save_as_preset(&self, name: String, overwrite: bool) -> BobResult<DatabaseAction<Preset>>;
}

impl IntoPresetData for GuildChannel {
    fn preset_data(self) -> BobResult<PresetData> {
        if self.kind != ChannelType::Voice {
            return Err(BobError::from_msg(ErrorKind::User, "Channel is not a voice channel"))
        }

        Ok(
            PresetData {
                bitrate: self.bitrate.bob_catch(ErrorKind::External, "Voice Channel has no bitrate")?,
                user_limit: self.user_limit,
                permissions: self.permission_overwrites
            }
        )
    }

    fn save_as_preset(&self, name: String, overwrite: bool) -> BobResult<DatabaseAction<Preset>> {
        Preset::save_raw(
            i64::from(self.guild_id),
            name,
            self.to_owned().preset_data()?,
            overwrite
        )
    }
}
