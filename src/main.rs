#![windows_subsystem = "windows"]
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt; 
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE,
};
use notify_rust::Notification;

#[derive(Serialize, Deserialize, Default)]
struct History {
    recent: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup paths dynamically
    let pictures_dir = dirs::picture_dir().ok_or("Could not find Pictures directory")?;
    let root_path = pictures_dir.join("Wallpapers");
    let history_path = root_path.join("history.toml");
    let mut rng = rand::thread_rng();

    // 2. Pick a GENRE first 
    let genres: Vec<PathBuf> = fs::read_dir(&root_path)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_dir()) 
        .collect();

    if genres.is_empty() {
        return Err("No genre folders found in Pictures/Wallpapers".into());
    }

    let selected_genre = genres.choose(&mut rng).unwrap();

    // 3. Use WalkDir ONLY on the selected genre to find all images inside it
    let entries: Vec<PathBuf> = WalkDir::new(selected_genre)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file()) // Ensure we only pick files
        .map(|e| e.path().to_owned())
        .collect();

    if entries.is_empty() {
        return Err("No images found in the selected genre".into());
    }

    // 4. Load history
    let mut history: History = fs::read_to_string(&history_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default();

    // 5. Select wallpaper with History Exclusion
    let fresh_options: Vec<PathBuf> = entries.iter()
        .filter(|p| !history.recent.contains(p))
        .cloned()
        .collect();

    let selected_wallpaper = if !fresh_options.is_empty() {
        fresh_options.choose(&mut rng).unwrap().clone()
    } else {
        history.recent.clear(); // Reset if this genre is exhausted
        entries.choose(&mut rng).unwrap().clone()
    };

    // 6. Update & save history
    history.recent.push(selected_wallpaper.clone());
    if history.recent.len() > 8 {
        history.recent.remove(0);
    }
    fs::write(&history_path, toml::to_string(&history)?)?;

    // 7. Convert path and set wallpaper
    let path_wide: Vec<u16> = OsStr::new(selected_wallpaper.as_os_str())
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(path_wide.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE,
        )?;
    }

    // 8. Send notification
    let genre_name = selected_genre.file_name().unwrap_or_default().to_string_lossy();
    let file_name = selected_wallpaper.file_name().unwrap_or_default().to_string_lossy();

    Notification::new()
        .summary("Wallpaper Updated")
        .body(&format!("Genre: {}\nFile: {}", genre_name, file_name))
        .appname("Wallpaper Picker")
        .show()?;

    Ok(())
}
