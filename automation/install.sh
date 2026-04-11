#!/bin/bash
# systemd integration & notification icons

#Create needed directories
INSTALL_DIR="~/config/systemd/user"
APP_DIR=".local/share/applications"
#link service files
ln -sf "$(pwd)/service/Wallpaper_Picker.service" $INSTALL_DIR
ln -sf "$(pwd)/service/Wallpaper_Picker.timer" $INSTALL_DIR

#link .desktop file for notification icon
ln -sf "$(pwd)/assets/Wallpaper_Picker.desktop" $APP_DIR

#Enable automation
systemctl --user daemon-reload
systemctl --user enable --now Wallpaper_Picker.timer

echo "Automation complete. Wallpaper_Picker is now active"
