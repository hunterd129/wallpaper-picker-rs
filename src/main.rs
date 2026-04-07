#![windows_subsystem = "windows"]
use std::fs;
use std::path::PathBuf;
use rand::seq::SliceRandom;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use serde::{Serialize, Deserialize};
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE,
};

use notify_rust::Notification;

// This must stay outside of main() to be found by the compiler
#[derive(Serialize, Deserialize, Default)]
struct History {
    recent: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_path = r"C:\Users\hunte\Pictures\Wallpapers";
    let history_path = PathBuf::from(root_path).join("history.toml");
    let mut rng = rand::thread_rng();

    // 1. Load History
    let mut history: History = fs::read_to_string(&history_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default();

    // 2. Find all sub-directories
    let folders: Vec<PathBuf> = fs::read_dir(root_path)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_dir())
        .collect();

    if folders.is_empty() {
        return Err("No subdirectories found in root directory".into());
    }

    let selected_folder = folders.choose(&mut rng).ok_or("Could not pick a folder")?;

    // 3. Get files in selected folder
    let entries: Vec<PathBuf> = fs::read_dir(selected_folder)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_file())
        .collect();

    if entries.is_empty() {
        return Err("No files found in folder".into());
    }

    // 4. Select Wallpaper with History Exclusion
    // Default to a random pick
    let mut selected_wallpaper = entries.choose(&mut rng).unwrap().clone();
    
    // Try to find options not in history
    let fresh_options: Vec<PathBuf> = entries.iter()
        .filter(|p| !history.recent.contains(p))
        .cloned() // This fixes the "cannot be built from &PathBuf" error
        .collect();

    if !fresh_options.is_empty() {
        selected_wallpaper = fresh_options.choose(&mut rng).unwrap().clone();
    }

    // 5. Update and Save History
   history.recent.push(selected_wallpaper.clone());

    // If we've reached 8 entries (the 7 old ones + the 1 we just added)
    if history.recent.len() >= 8 {
        history.recent.clear(); // Wipe the list clean for the next cycle
    }

    fs::write(&history_path, toml::to_string(&history)?)?;    fs::write(&history_path, toml::to_string(&history)?)?;

    // 6. Convert path and set wallpaper
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

    // 7. Send notification
    let folder_name = selected_folder.file_name().unwrap_or_default().to_string_lossy();
    let file_name = selected_wallpaper.file_name().unwrap_or_default().to_string_lossy();

    Notification::new()
        .summary("Wallpaper Updated")
        .body(&format!("Theme: {}\nFile: {}", folder_name, file_name))
        .appname("Wallpaper Picker")
        .icon("image-png")
        .show()?;

    Ok(())
}
