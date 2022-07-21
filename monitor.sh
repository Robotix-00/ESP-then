#!/usr/bin/env bash

read -p "interface: " interface


sudo ifconfig $interface down
sudo iwconfig $interface mode monitor

# leave me alone, it doesnt work without it
sudo rfkill unblock wifi

sudo ifconfig $interface up
