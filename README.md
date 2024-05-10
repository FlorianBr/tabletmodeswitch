# Tablet mode switcher

## What

A Daemon to automatically switch between tablet and desktop mode. Written for a Lenovo X1 Yoga.

## How

The program listens on a event-input and when the mode changes it calls the batch-file `change_mode.sh` with the argument "T" for tablet or "L" for laptop mode. All setting modifications, program starts/stops etc. can be done in the batch without changing the main program.

The running user must be member of the "input" group to have the permissions to read the event files.

Usage:

```
modeswitcher -e /dev/input/event7
```

Note:
The event number may change, better use the matching symlink in by-path: `ls -al /dev/input/by-path`. In case of my Yoga it is `/dev/input/by-path/platform-INTC1070:00-event`
