mod models;

use models::{CacheRecord, Settings};
use ron::ser::{to_string_pretty, PrettyConfig};
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let bookmark_config_path = dirs::home_dir()
        .expect("Could not find home directory.")
        .join(".bkmk_config");
    let folder_exists = bookmark_config_path.exists();

    if !folder_exists {
        initialize_files(&bookmark_config_path);
    }

    let mut settings_file = File::open(bookmark_config_path.join("settings.ron"))
        .expect("Could not load settings file.");

    let settings = Settings::load(&mut settings_file).expect("Could not load settings file.");

    println!("Could find folder? {}", folder_exists);
    println!("{:?}", settings);

    Ok(())
}

fn initialize_files(path: &PathBuf) {
    create_dir(path).expect("Could not create a config directory.");
    let mut cache_file =
        File::create(path.join("cache.ron")).expect("Could not create a cache file.");
    let cache: Vec<CacheRecord> = Vec::new();
    let pretty = PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let data = to_string_pretty(&cache, pretty).expect("Serialization failed.");

    cache_file
        .write(data.as_bytes())
        .expect("Could not write to cache file.");

    let settings = models::Settings::new();
    let mut settings_file =
        File::create(path.join("settings.ron")).expect("Could not create a cache file.");
    settings_file
        .write(settings.serialize().as_bytes())
        .expect("Could not write to settings file.");
}
