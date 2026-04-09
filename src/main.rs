use gio::prelude::*;
use gio::Settings;
use notify_rust::Notification;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Default)]
struct History {
    previous: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let root_path = format!("{}/Pictures/Wallpapers", home);
    let history_path = PathBuf::from(&home).join(".wallpaper_history.toml");

    //Collect all images
    let mut entries: Vec<PathBuf> = WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    if entries.is_empty() {
        return Err("No wallpaper files found in ~/Pictures/Wallpapers".into());
    }

    //Load history and exclude recent choices
    let mut history: History = if history_path.exists() {
        let content = fs::read_to_string(&history_path)?;
        toml::from_str(&content).unwrap_or_default()
    } else {
        History::default()
    };

    //Filter for options not in history
    let fresh_options: Vec<PathBuf> = entries
        .iter()
        .filter(|p| {
            let path_str = p.to_string_lossy().to_string();
            !history.previous.contains(&path_str)
    })
    .cloned()
    .collect();

    //Pick a wallpaper
    let mut rng = rand::thread_rng();
    let selected_wallpaper = if !fresh_options.is_empty() {
        fresh_options.choose(&mut rng).unwrap().clone()
    } else {
        history.previous.clear();
        entries.choose(&mut rng).unwrap().clone()
    };

    let wallpaper_path = selected_wallpaper.to_str().ok_or("Invalid path")?;
    let wallpaper_uri = format!("file://{}", wallpaper_path);

    //Update history
    history.previous.push(wallpaper_path.to_string());
    if history.previous.len() > 8 {
        history.previous.remove(0);
    }
    fs::write(&history_path, toml::to_string(&history)?)?;

    //apply via Gsettings
    let settings = Settings::new("org.gnome.desktop.background");
    settings.set_string("picture-uri", &wallpaper_uri)?;
    settings.set_string("picture-uri-dark", &wallpaper_uri)?; //dark mode
    settings.apply();

    //Notify
    let file_name = selected_wallpaper
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("Unknown Image");

    Notification::new()
        .summary("Wallpaper Updated")
        .body(&format!("New look: {}", file_name))
        .appname("Wallpaper Picker")
        .timeout(5000)
        .show()?;

    Ok(())
}

