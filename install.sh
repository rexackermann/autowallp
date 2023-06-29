#!/bin/env bash
git clone https://github.com/rexackermann/autowallp.git
cd autowallp
sudo cp -rfv ./autowallp/autowallp /usr/local/bin/
sudo cp -rfv ./autowallp/wallch /usr/local/bin/
cp -rfv ./autowallp "${XDG_CONFIG_HOME:-"$HOME"/.config}/"
sudo chmod +x /usr/local/bin/autowallp
mkdir ~/.config/autostart/
cp -rfv autowallp.desktop ~/.config/autostart/
