#![windows_subsystem = "windows"]
use std::fs;
use std::path::PathBuf;
use rand::seq::SliceRandom;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE,
};

use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Define the parent directory
    let root_path = r"C:\Users\hunte\Pictures\Wallpapers";
    let mut rng = rand::thread_rng();

   //Find all sub-directories within the parent
   let folders: Vec<PathBuf> = fs::read_dir(root_path)?
       .filter_map(|res| res.ok())
       .map(|e| e.path())
       .filter(|path| path.is_dir()) //Only include folders
       .collect();

    if folders.is_empty() {
        return Err("No subdirectories found in root directory".into());
    }

    //Pick a random folder from the dynamic list
    let selected_folder = folders.choose(&mut rng).unwrap();

    //Get all files in selected folder
    let entries : Vec<PathBuf> = fs::read_dir(selected_folder)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_file())
        .collect();

    let selected_wallpaper = entries.choose(&mut rng).ok_or("No files found in folder")?;

    //Convert path and set wallpaper
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

    //Send notification
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
    
