use core::str::FromStr;
use regex::Regex;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum BookmarkCli {
    /// Show all available bookmarks
    List,
    /// Add a new bookmark
    Add,
    /// Remove a bookmark
    Remove,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cache(Vec<CacheRecord>);

impl Cache {
    pub fn new() -> Cache {
        Cache(Vec::new())
    }

    pub fn get_records(&self) -> &Vec<CacheRecord> {
        &self.0
    }

    pub fn load_from_file(file: &mut File) -> std::result::Result<Cache, ron::Error> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to read file to string.");
        if buf == "[]" {
            Ok(Cache::new())
        } else {
            ron::de::from_str(buf.as_str())
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CacheRecord {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum KeyBinds {
    Default,
    Emacs,
    Vim,
}

#[derive(Debug)]
pub struct Settings {
    pub ignore: Vec<Regex>,
    pub key_binds: KeyBinds,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            ignore: Vec::new(),
            key_binds: KeyBinds::Default,
        }
    }

    pub fn serialize(self) -> String {
        let pretty = PrettyConfig::new()
            .with_depth_limit(2)
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true);
        let ser = SerializableSettings::from(self);
        to_string_pretty(&ser, pretty).expect("Serialization failed.")
    }

    pub fn load_from_file(file: &mut File) -> std::result::Result<Settings, regex::Error> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to read file to string.");
        let deser: SerializableSettings =
            ron::de::from_str(buf.as_str()).expect("Failed to read file.");
        Settings::try_from(deser)
    }
}

impl TryFrom<SerializableSettings> for Settings {
    type Error = regex::Error;
    fn try_from(item: SerializableSettings) -> Result<Self, Self::Error> {
        let ignore = item
            .ignore
            .iter()
            .map(|x| Regex::from_str(x))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Settings {
            ignore,
            key_binds: item.key_binds,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct SerializableSettings {
    ignore: Vec<String>,
    key_binds: KeyBinds,
}

impl From<Settings> for SerializableSettings {
    fn from(value: Settings) -> Self {
        SerializableSettings {
            ignore: value.ignore.iter().map(|x| x.to_string()).collect(),
            key_binds: value.key_binds,
        }
    }
}
