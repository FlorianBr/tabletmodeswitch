#!/bin/sh

if [ $1 = "T" ]; then
    echo "Changing mode to TABLET"
    florence show
    xrandr --output eDP-1  --rotate inverted
    xinput set-prop "Wacom HID 5285 Pen stylus" --type=float "Coordinate Transformation Matrix" -1 0 1 0 -1 1 0 0 1
    xinput set-prop "Wacom HID 5285 Finger touch" --type=float "Coordinate Transformation Matrix" -1 0 1 0 -1 1 0 0 1
fi

if [ $1 = "L" ]; then
    echo "Changing mode to LAPTOP"
    florence hide
    xrandr --output eDP-1  --rotate normal
    xinput set-prop "Wacom HID 5285 Pen stylus" --type=float "Coordinate Transformation Matrix" 0 0 0 0 0 0 0 0 0
    xinput set-prop "Wacom HID 5285 Finger touch" --type=float "Coordinate Transformation Matrix" 0 0 0 0 0 0 0 0 0
fi