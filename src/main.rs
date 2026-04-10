use gio::Settings;
use gio::prelude::SettingsExt;
use notify_rust::Notification;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Default)]
struct History {
    previous: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let root_path = home.join("Pictures/Wallpapers");
    let history_root = home.join(".local/share/Wallpaper_Picker");
    let history_path = history_root.join("history.toml");

    if !history_root.exists() {
        fs::create_dir_all(&history_root)?;
    }

    let mut rng = rand::thread_rng();

    // 1. Pick a GENRE first
    let genres: Vec<PathBuf> = fs::read_dir(&root_path)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_dir()) 
        .collect();

    if genres.is_empty() {
        return Err("No genre folders found in ~/Pictures/Wallpapers".into());
    }

    let selected_genre = genres.choose(&mut rng).unwrap();

    // 2. Use WalkDir ONLY on the selected genre
    let entries: Vec<PathBuf> = WalkDir::new(selected_genre)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    if entries.is_empty() {
        return Err("No images found in the selected genre".into());
    }

    // 3. Load history and filter for "fresh" options in this genre
    let mut history: History = if history_path.exists() {
        let content = fs::read_to_string(&history_path)?;
        toml::from_str(&content).unwrap_or_default()
    } else {
        History::default()
    };

    let fresh_options: Vec<PathBuf> = entries
        .iter()
        .filter(|p| {
            let path_str = p.to_string_lossy().to_string();
            !history.previous.contains(&path_str)
        })
        .cloned()
        .collect();

    // 4. Final Selection
    if history.previous.len() >= 7 {
        history.previous.clear();
    }

    let selected_wallpaper = if !fresh_options.is_empty() {
        fresh_options.choose(&mut rng).unwrap().clone()
    } else {
        entries.choose(&mut rng).unwrap().clone()
    };

    let wallpaper_path = selected_wallpaper.to_str().ok_or("Invalid path")?;
    let wallpaper_uri = format!("file://{}", wallpaper_path);

    // 5. Update and Save History
    history.previous.push(wallpaper_path.to_string());
    fs::write(&history_path, toml::to_string(&history)?)?;

    // 6. Apply to GNOME
    let settings = Settings::new("org.gnome.desktop.background");
    settings.set_string("picture-uri", &wallpaper_uri)?;
    settings.set_string("picture-uri-dark", &wallpaper_uri)?;

    // 7. Notify
    let genre_name = selected_genre.file_name().unwrap_or_default().to_string_lossy();
    let file_name = selected_wallpaper.file_name().unwrap_or_default().to_string_lossy();

    Notification::new()
        .summary("Wallpaper Updated")
        .body(&format!("Genre: {}\nFile: {}", genre_name, file_name))
        .appname("Wallpaper Picker")
        .icon("media-playlist-shuffle")
        .timeout(5000)
        .show()?;

    Ok(())
}
