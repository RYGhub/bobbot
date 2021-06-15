use std::env::{current_dir};
use std::path::{PathBuf};
use std::fs::{File, read_dir, create_dir_all};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serenity::model::prelude::{PermissionOverwrite, VideoQualityMode, GuildId, GuildChannel};
use crate::basics::result::{BobResult, BobError};
use crate::basics::channel;
use crate::basics::permows::{clone_from_guildchannel};


/// Get the current working directory.
fn cwd() -> BobResult<PathBuf> {
    current_dir()
        .map_err(|_| BobError {msg: "Couldn't get current working directory"})
}


/// A container for serializing Discord channel data.
#[derive(Serialize, Deserialize, Debug)]
pub struct BobPreset {
    pub bitrate: u32,
    pub user_limit: Option<u32>,
    pub video_quality: VideoQualityMode,
    pub permows: Vec<PermissionOverwrite>,
}


impl BobPreset {
    /// Create a [BobPreset] from the given voice [GuildChannel].
    pub fn create_from_voice_channel(channel: &GuildChannel) -> BobResult<Self> {
        Ok(
            BobPreset {
                bitrate: channel::get_bitrate(&channel)?,
                user_limit: channel::get_user_limit(&channel)?,
                video_quality: channel::get_video_quality(&channel),
                permows: clone_from_guildchannel(&channel),
            }
        )
    }

    /// Get the main presets directory (`./presets/`).
    pub fn presets_dir() -> BobResult<PathBuf> {
        Ok(cwd()?.join("presets"))
    }


    /// Get the presets directory of the given guild (`./presets/{guild_id}/`).
    pub fn guild_presets_dir(guild_id: &GuildId) -> BobResult<PathBuf> {
        Ok(BobPreset::presets_dir()?.join(format!("{}", &guild_id)))
    }

    /// Get the preset filename for the given guild and name (`./presets/{guild_id}/{preset_name}.toml`).
    pub fn guild_preset_filename(guild_id: &GuildId, preset_name: &str) -> BobResult<PathBuf> {
        Ok(BobPreset::guild_presets_dir(guild_id)?.join(format!("{}.toml", &preset_name)))
    }

    /// Get a list of available preset filenames for the given guild.
    pub fn guild_presets_file_list(guild_id: &GuildId) -> BobResult<Vec<PathBuf>> {
        let gpd = BobPreset::guild_presets_dir(guild_id)?;

        let preset_files = read_dir(gpd)
            .map_err(|_| BobError {msg: "Could not read guild presets directory contents"})?;

        let mut mapped_files = vec![];
        for preset_file in preset_files {
            if let Err(_) = &preset_file {
                return Err(BobError {msg: "Could not read guild preset file"})
            }
            mapped_files.push(preset_file.unwrap().path());
        }

        Ok(mapped_files)
    }

    /// Read the given file into a [BobPreset].
    pub fn read_file(mut file: File) -> BobResult<Self> {
        let mut serialized = vec![];

        file.read_to_end(&mut serialized)
            .map_err(|_| BobError {msg: "Could not read preset file contents"})?;

        toml::from_slice(&serialized)
            .map_err(|_| BobError {msg: "Could not deserialize preset"})
    }

    /// Read the [BobPreset] for the given guild and name.
    pub fn read_guild(guild_id: &GuildId, name: &str) -> BobResult<Self> {
        let filename = BobPreset::guild_preset_filename(&guild_id, name)?;

        let file = File::open(&filename)
            .map_err(|_| BobError {msg: "Could not open preset file"})?;

        BobPreset::read_file(file)
    }

    /// Write the [BobPreset] into the given file.
    pub fn write_file(&self, mut file: File) -> BobResult<()> {
        let serialized = toml::to_string(&self)
            .map_err(|_| BobError {msg: "Could not serialize preset"})?
            .into_bytes();

        file.write_all(&serialized)
            .map_err(|_| BobError {msg: "Could not write preset file"})?;

        Ok(())
    }

    /// Write the given [BobPreset] for the given guild and name.
    pub fn write_guild(&self, guild_id: &GuildId, preset_name: &str) -> BobResult<()> {
        let filename = BobPreset::guild_preset_filename(&guild_id, preset_name)?;

        create_dir_all(filename.clone().parent().unwrap())
            .map_err(|_| BobError {msg: "Could not create preset directory"});

        let file = File::create(&filename)
            .map_err(|_| BobError {msg: "Could not create preset file"})?;

        debug!("Writing guild preset {} of <G{}>", &preset_name, &guild_id);
        self.write_file(file)?;
        Ok(())
    }
}
