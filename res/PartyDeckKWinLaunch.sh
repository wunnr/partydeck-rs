#!/bin/bash

resolution=$(xrandr | grep '*' | awk '{print $1}')
width=$(echo $resolution | cut -d 'x' -f 1)
height=$(echo $resolution | cut -d 'x' -f 2)

kwin_wayland --xwayland --width $width --height $height --exit-with-session "konsole -e ./partydeck-rs --fullscreen"
