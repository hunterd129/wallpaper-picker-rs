use std::fs;
use std::path::PathBuf;
use rand::seq::SliceRandom;
use gio::prelude::*;
use gio::Settings;
use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Updated file paths for POSIX systems
let folders = vec![
    "/home/hunter/Pictures/Wallpapers/Architecture Wallpapers",
    "/home/hunter/Pictures/Wallpapers/Libadwaita Wallpapers",
    "/home/hunter/Pictures/Wallpapers/Yuzusoft Wallpapers",
];

let mut rng = rand::thread_rng();

//Pick a random folder
let selected_folder = folders.choose(&mut rng).ok_or("folder list is empty")?;

//collect files
let entries = fs::read_dir(selected_folder)?
    .filter_map(|res| re.ok())
    .map(|e| e.path())
    .filter(|path| path.is_file())
    .collect::<Vec<PathBuf>>();

    if entries.is_empty() {
        return Err("No files found in the selected folder".into());
    }

//pick the wallpaper
let selected_wallpaper = entries.choose(&mut rng).unwrap();
let wallpaper_path = selected_wallpaper.to_str().ok_or("Invalid path")?;

let wallpaper_uri = format!("file//{}", wallpaper_path);

//set wallpaper via gsettings
let settings = Settings::new("org.gnome.desktop.background");
settings.set_string("picture-uri", &wallpaper_uri)?;
settings.set_string("picture-uri-dark", &wallpaper_uri)?;

//apply changes
settings.apply();

//send notification
let file_name = selected_wallpaper
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("Unknown Image");

Notification::new()
    .summary("Wallpaper Updated")
    .body(&format!("New look: {}", file_name))
    .appname("wallpaperPicker")
    .timeout(5000)
    .show()?;

ok(())
}
