This is a rust solution to automatically switching your wallpapers in Windows 10 by first randomizing which folder it will pick from, then randomizing the individual file inside of the chosen folder.
    
    - be sure to replace [path\to\your\wallpapers] with the actual file path
    - cargo run
once you have created your exe, move it to where you would like to keep it, then open task scheduler, click 'Create Basic Task', set the trigger to when you want it to swap, (e.g. on log on, weekly, etc) then for actions, pick run a program and point it to where you put the .exe.
