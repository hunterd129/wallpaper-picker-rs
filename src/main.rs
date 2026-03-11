#![windows_subsystem = "windows"]
use std::fs;
use std::path::PathBuf;
use rand::seq::SliceRandom;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. The folders from your PS1 script
    let folders = vec![
        r"C:\Users\hunte\Pictures\Wallpapers\Architecture Wallpapers",
        r"C:\Users\hunte\Pictures\Wallpapers\Libadwaita Wallpapers",
        r"C:\Users\hunte\Pictures\Wallpapers\Yuzusoft Wallpapers",
    ];

    let mut rng = rand::thread_rng();

    // 2. Pick a random folder
    let selected_folder = folders.choose(&mut rng).expect("Folder list is empty");

    // 3. Get all files and pick a random one
    let entries = fs::read_dir(selected_folder)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>();

    let selected_wallpaper = entries.choose(&mut rng).ok_or("No files found in folder")?;

    // 4. Convert the path to the "Wide String" format user32.dll expects
    let path_wide: Vec<u16> = OsStr::new(selected_wallpaper.as_os_str())
        .encode_wide()
        .chain(std::iter::once(0)) // Null terminator
        .collect();

    // 5. The "Spiteful" call to user32.dll
    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(path_wide.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE,
        )?;
    }

    println!("Success! Wallpaper set to: {:?}", selected_wallpaper);
    Ok(())
}
