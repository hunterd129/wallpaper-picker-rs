# Install the system headers required by the 'gio' crate
pacman -S glib2 base-devel

pacman -S rust

# build
cargo run

#automation
refer to systemd-wallpaper-swap.service repo
