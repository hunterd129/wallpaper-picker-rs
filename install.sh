#!/bin/bash
# install.sh

# Define paths
SERVICE_DIR="$HOME/.config/systemd/user"
APP_DIR="$HOME/.local/share/applications"
BIN_DIR="$HOME/.local/bin"

#Build binary
echo "Building Wallpaper_Shuffler..."
cargo build --release || { echo "Build failed. Check your Rust environment."; exit 1; }
# Ensure dirs exist
mkdir -p "$SERVICE_DIR" "$APP_DIR" "$BIN_DIR"

#Move the binary
cp "target/release/Wallpaper_Shuffler" "$BIN_DIR"

# 1. Generate the service file
cat <<EOF > "$SERVICE_DIR/Wallpaper_Shuffler.service"
[Unit]
Description=Trigger Wallpaper swap

[Service]
Type=oneshot
ExecStart=$BIN_DIR/Wallpaper_Shuffler

[Install]
WantedBy=default.target
EOF

# 2. Generate the .desktop file
cat <<EOF > "$APP_DIR/Wallpaper_Shuffler.desktop"
[Desktop Entry]
Name=Wallpaper_Shuffler
Exec=$BIN_DIR/Wallpaper_Shuffler
Icon=media-playlist-shuffle
Type=Application
Terminal=false
Categories=Utility
EOF

#Generate the service timer
cat <<EOF > "$SERVICE_DIR/Wallpaper_Shuffler.timer"
[Unit]
Description=Schedule for wallpaper swap

[Timer]
OnCalendar=daily
Persistent=true
Unit=Wallpaper_Shuffler.service

[Install]
WantedBy=timers.target
EOF

# Reload and enable
systemctl --user daemon-reload
systemctl --user enable --now Wallpaper_Shuffler.timer
