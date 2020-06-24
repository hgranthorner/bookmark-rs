use regex::Regex;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;
use std::convert::TryFrom;

#[derive(Serialize, Debug)]
pub enum KeyBinds {
    Default,
    Emacs,
    Vim,
}

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
        let ser = SerializableSettings::try_from(self)
            .expect("Failed to convert settings to serializable form.");
        println!("{:?}", ser);
        to_string_pretty(&ser, pretty).expect("Serialization failed.")
    }
}

#[derive(Serialize, Debug)]
struct SerializableSettings {
    ignore: Vec<String>,
    key_binds: KeyBinds,
}

impl TryFrom<Settings> for SerializableSettings {
    type Error = &'static str;

    fn try_from(value: Settings) -> Result<Self, Self::Error> {
        Ok(SerializableSettings {
            ignore: value.ignore.iter().map(|x| x.to_string()).collect(),
            key_binds: value.key_binds,
        })
    }
}
