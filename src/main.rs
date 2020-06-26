mod models;

use models::{BookmarkCli, Cache, CacheRecord, Settings};
use ron::ser::{to_string, to_string_pretty, PrettyConfig};
use std::convert::TryFrom;
use std::env;
use std::fs::{create_dir, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    let args = BookmarkCli::from_args();
    let (mut cache, _settings) = load_cache_and_settings();
    match args {
        BookmarkCli::List => {
            cache.list_bookmarks();
        }
        BookmarkCli::Add => {
            let mut name = String::from("");
            let mut description = String::from("");
            let mut path = String::from("");
            let path_buf: PathBuf;

            match std::env::current_dir() {
                Ok(dir) => {
                    let s = dir.to_str().unwrap_or("").split('/').last().unwrap_or("");
                    print!("Name [{:?}]: ", s);
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut name)?;
                    if name == "\n" {
                        name = s.to_string();
                    }
                    print!("Description []: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut description)?;
                    if description == "\n" {
                        description = "".to_string();
                    }
                    print!("Path [{:?}]: ", dir);
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut path)?;
                    if path == "\n" {
                        path_buf = dir;
                    } else {
                        path_buf = PathBuf::try_from(path).expect("Invalid path.");
                    }

                    cache.get_mut_records().push(CacheRecord {
                        name: name.replace("\n", ""),
                        description: description.replace("\n", ""),
                        path: path_buf,
                    });

                    println!("{:?}", cache);
                    save_cache_to_file(&cache);
                }
                Err(_) => {
                    print!("Name: ");
                }
            }
        }
        BookmarkCli::Remove => {}
        BookmarkCli::Go { target } => {
            let record = cache
                .get_records()
                .iter()
                .find(|x| x.name == target)
                .expect("Could not find matching name.");
            env::set_current_dir(&record.path).expect("Failed to set current directory.");
        }
    };
    Ok(())
}

fn save_cache_to_file(cache: &Cache) {
    let mut cache_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(
            dirs::home_dir()
                .expect("Could not find home directory.")
                .join(".bkmk_config")
                .join("cache.ron"),
        )
        .expect("Could not open cache file with write access.");
    // let pretty = PrettyConfig::new()
    //     .with_depth_limit(2)
    //     .with_separate_tuple_members(true)
    //     .with_enumerate_arrays(true);

    let ser = to_string(cache.get_records()).expect("Serialization failed.");
    cache_file
        .write(ser.as_bytes())
        .expect("Failed to write to cache file.");
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
    let cache = Cache::load_from_file(&mut cache_file);
    (cache, settings)
}

fn initialize_files(path: &PathBuf) -> (File, File) {
    // I would like this to work by creating the file, writing to it,
    // and then accessing the data in the file later (deserializing it).
    // Unfortunately it seems like this doesn't work, and we can't access
    // the newly created file. The current work around is to create the file,
    // write to it, close it, then re-open it.
    create_dir(path).expect("Could not create a config directory.");
    {
        let mut cache_file = open_cache_file(path);
        let cache: Vec<CacheRecord> = Vec::new();
        let pretty = PrettyConfig::new()
            .with_depth_limit(2)
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true);
        let data = to_string_pretty(&cache, pretty).expect("Serialization failed.");

        cache_file
            .write(data.as_bytes())
            .expect("Could not write to cache file.");
        cache_file.flush().expect("Failed to flush cache file.");

        let settings = models::Settings::new();
        let mut settings_file = open_settings_file(path);

        settings_file
            .write(settings.serialize().as_bytes())
            .expect("Could not write to settings file.");
        settings_file
            .flush()
            .expect("Failed to flush settings file.");
    }

    let cache_file = open_cache_file(path);
    let settings_file = open_settings_file(path);
    (cache_file, settings_file)
}

fn open_cache_file(path: &PathBuf) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.join("cache.ron"))
        .expect("Could not create a cache file.")
}

fn open_settings_file(path: &PathBuf) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.join("settings.ron"))
        .expect("Could not create a settings file.")
}
