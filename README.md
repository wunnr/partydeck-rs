
### `New input build`

this is for developing and testing of a new input system. Users will be able to place multiple devices on one instance, including keyboards and mice on top of gamepads.

You'll need a custom version of gamescope that supports evdev input holding. This can be found at https://github.com/davidawesome02-backup/gamescope. Build the project, and place the `gamescope` binary in the res folder of the partydeck build. By the time this gets merged to main, I'll have figured out how to distribute this custom binary, or Valve will have merged input holding into upstream gamescope.
