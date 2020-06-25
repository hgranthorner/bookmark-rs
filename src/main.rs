mod models;

use models::{BookmarkCli, Cache, CacheRecord, Settings};
use ron::ser::{to_string_pretty, PrettyConfig};
use std::cmp::max;
use std::fs::{create_dir, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    let args = BookmarkCli::from_args();
    println!("{:?}", args);
    let (cache, settings) = load_cache_and_settings();
    println!("{:?}", settings);
    println!("{:?}", cache);
    match args {
        BookmarkCli::List => {
            println!("Available bookmarks:");
            let (len_name, len_description, len_path) =
                cache.get_records().iter().fold((7, 14, 7), |acc, rec| {
                    (
                        max(acc.0, rec.name.len()),
                        max(acc.1, rec.description.len()),
                        max(acc.2, rec.path.to_str().unwrap_or("").len()),
                    )
                });
            print_fixed_width("Name:", len_name);
            print!(" | ");
            print_fixed_width("Description:", len_description);
            print!(" | ");
            print_fixed_width("Path:", len_path);
            print!("\n");
            println!("{}", "-".repeat(len_name + len_description + len_path + 6));
        }
        BookmarkCli::Add => {}
        BookmarkCli::Remove => {}
    };
    Ok(())
}

fn load_cache_and_settings() -> (Cache, Settings) {
    let bookmark_config_path = dirs::home_dir()
        .expect("Could not find home directory.")
        .join(".bkmk_config");
    let folder_exists = bookmark_config_path.exists();

    let (mut cache_file, mut settings_file) = if !folder_exists {
        initialize_files(&bookmark_config_path)
    } else {
        (
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(&bookmark_config_path.join("cache.ron"))
                .expect("Could not open cache file."),
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(&bookmark_config_path.join("settings.ron"))
                .expect("Could not open settings file."),
        )
    };

    let settings =
        Settings::load_from_file(&mut settings_file).expect("Could not load settings file.");
    let cache = Cache::load_from_file(&mut cache_file).expect("Could not load cache file.");
    (cache, settings)
}

fn initialize_files(path: &PathBuf) -> (File, File) {
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
    (cache_file, settings_file)
}

fn print_fixed_width(s: &str, n: usize) {
    if s.len() < n {
        print!("{}{}", s, " ".repeat(n - s.len()));
    } else {
        print!("{}", s);
    }
}
