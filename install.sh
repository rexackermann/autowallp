#!/bin/bash

#git clone 
sudo cp -rfv target/release/autowallp /usr/local/bin/
cp -rfv autowallp ~/.config/autowallp
sudo chmod +x /usr/local/bin/autowallp
mkdir ~/.config/autostart/
cp -rfv autowallp.desktop ~/.config/autostart/