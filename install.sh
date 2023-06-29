#!/usr/bin/env bash

git clone https://github.com/rexackermann/autowallp.git
cd autowallp
sudo cp -rfv ./autowallp/autowallp /usr/local/bin/
cp -rfv ./autowallp ~/.config/autowallp
sudo chmod +x /usr/local/bin/autowallp
mkdir ~/.config/autostart/
cp -rfv autowallp.desktop ~/.config/autostart/
