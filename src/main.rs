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
use notify_rust::{Notification, Timeout};

#[derive(Serialize, Deserialize, Default)]
struct History {
    recent: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find Home")?;
    let pictures_dir = dirs::picture_dir().ok_or("Could not find Pictures")?;
    let root_path = pictures_dir.join("Wallpapers");
    let history_root = home_dir.join(".local/share/Wallpaper_Shuffler");
    let history_path = history_root.join("history.toml");

    if !history_root.exists() {
        fs::create_dir_all(&history_root)?;
    }
    let mut rng = rand::thread_rng();

    let genres: Vec<PathBuf> = fs::read_dir(&root_path)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_dir()) 
        .collect();

    if genres.is_empty() {
        return Err("No images found in ~/Pictures/Wallpapers".into());
    }

    let chosen_genre = genres.choose(&mut rng).unwrap();

    let entries: Vec<PathBuf> = WalkDir::new(chosen_genre)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file()) // Ensure we only pick files
        .map(|e| e.path().to_owned())
        .collect();

    if entries.is_empty() {
        return Err("No images found in the chosen genre".into());
    }

    let mut history: History = fs::read_to_string(&history_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default();

    let fresh_options: Vec<PathBuf> = entries.iter()
        .filter(|p| !history.recent.contains(p))
        .cloned()
        .collect();

    let chosen_image = if !fresh_options.is_empty() {
        fresh_options.choose(&mut rng).unwrap().clone()
    } else {
        history.recent.clear();
        entries.choose(&mut rng).unwrap().clone()
    };

    // 6. Update & save history
    if history.recent.len() >= 7 {
        history.recent.remove(0);
    }
    history.recent.push(chosen_image.clone());
    fs::write(&history_path, toml::to_string_pretty(&history)?)?;

    // Set Wallpaper
    let path_wide: Vec<u16> = OsStr::new(chosen_image.as_os_str())
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
    let genre = chosen_genre.file_name().unwrap_or_default().to_string_lossy();
    let file = chosen_image.file_name().unwrap_or_default().to_string_lossy();

    Notification::new()
        .summary("Wallpaper Updated")
        .body(&format!("Genre: {}\nFile: {}", genre, file))
        .appname("Wallpaper Shuffler")
        .image_path(&chosen_image.to_str().unwrap_or_default())
        .timeout(Timeout::Milliseconds(5000))
        .show()?;

    Ok(())
}
