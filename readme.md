# About

This is a recursive solution to automating wallpaper shuffle while also supporting multiple directories by starting in ~\Pictures\Wallpapers, randomly picking any of the available choices, then using walkdir to find the new wallpaper from there.

The primary issue I have with how Windows natively handles wallpaper shuffling is that it either requires you to organize your wallpapers into a lump sum, or to manually choose which directory it will pick from which is tedious to say the least.

The primary goal of this code is to randomly choose a genre category, then select an image from within that category while also keeping a rolling list of the seven most recent images in order to avoid seeing the same image within the span of a week.

# Files
- The configuration file for weighted selection is found in ~\.config\Wallpaper_Shuffler\config.toml
- History is located in ~\.local\share\Wallpaper_Shuffler\history.toml

# Automation
- Download the source code, then run `cargo build --release`.
- In order to automate it, create a basic task with Windows Task Scheduler, tell it to run the .exe, and set the interval to however often you want.

# Dependencies
Requires:
- Rust
- Visual studio

# Note
This assumes that you put your wallpapers and any sub-directories for wallpapers specifically in ~\Pictures\Wallpapers. If you do not, you will need to change the source code to point to somewhere else.
