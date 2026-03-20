# Install the system headers required by the 'gio' crate
pacman -S glib2 base-devel
pacman -S rust

# Clone and build
git clone -b linux [your-repo-url]
cd [your-repo-name]
cargo run
