use core::str::FromStr;
use regex::Regex;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::convert::{From, TryFrom};
use std::fs::File;
use std::io::Read;
use std::io::Write;
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
    /// Go to the bookmarked directory
    Go {
        #[structopt(short, long)]
        target: String,
    },
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
    pub fn get_mut_records(&mut self) -> &mut Vec<CacheRecord> {
        &mut self.0
    }

    pub fn load_from_file(file: &mut File) -> Cache {
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to read file to string.");
        if buf == "[]" {
            Cache::new()
        } else {
            Cache(
                ron::de::from_str(buf.as_str())
                    .expect("Could not convert string to Vec<CacheRecord>."),
            )
        }
    }
    pub fn list_bookmarks(&self) {
        println!("Available bookmarks:");
        let (len_name, len_description, len_path) =
            self.get_records().iter().fold((7, 14, 7), |acc, rec| {
                (
                    max(acc.0, rec.name.len()),
                    max(acc.1, rec.description.len()),
                    max(acc.2, rec.path.to_str().unwrap_or("").len()),
                )
            });
        println!("{}", "-".repeat(len_name + len_description + len_path + 6));
        print_fixed_width("Name:", len_name);
        print!(" | ");
        print_fixed_width("Description:", len_description);
        print!(" | ");
        print_fixed_width("Path:", len_path);
        print!("\n");
        println!("{}", "-".repeat(len_name + len_description + len_path + 6));
        self.get_records().iter().for_each(|r| {
            print_fixed_width(&r.name, len_name);
            print!(" | ");
            print_fixed_width(&r.description, len_description);
            print!(" | ");
            print_fixed_width(&r.path.to_str().unwrap_or(""), len_path);
            print!("\n");
        })
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
        file.flush()
            .expect("Failed to flush file when reading to string.");
        file.sync_all().expect("Failed to sync data with the disk.");
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

fn print_fixed_width(s: &str, n: usize) {
    if s.len() < n {
        print!("{}{}", s, " ".repeat(n - s.len()));
    } else {
        print!("{}", s);
    }
}
