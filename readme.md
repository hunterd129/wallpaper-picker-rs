# Install the system headers required by the 'gio' crate
sudo pacman -S glib2 base-devel

# Clone and build
git clone -b linux [your-repo-url]
cd [your-repo-name]
cargo run
