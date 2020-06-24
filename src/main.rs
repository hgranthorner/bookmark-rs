mod models;

use std::fs::{create_dir, File};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let bookmark_config_path = dirs::home_dir()
        .expect("Could not find home directory.")
        .join(".bkmk_config");
    let folder_exists = bookmark_config_path.exists();
    if !folder_exists {
        let settings = models::Settings::new();
        create_dir(&bookmark_config_path).expect("Could not create a config directory.");
        File::create(&bookmark_config_path.join("cache.ron"))
            .expect("Could not create a cache file.");
        let mut settings_file = File::create(&bookmark_config_path.join("settings.ron"))
            .expect("Could not create a cache file.");
        settings_file
            .write(settings.serialize().as_bytes())
            .expect("Could not write to file.");
    }
    println!("{:?}", bookmark_config_path);
    println!("{}", folder_exists);

    Ok(())
}
