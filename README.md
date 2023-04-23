# WASMBoi

This is my GB emulator written in Rust. There are many like it, but this one is my own.

You better believe it's cycle accurate.

## Stream

I stream nearly all development on twitch, feel free to tune in: https://www.twitch.tv/gasolinewaltz

There is no schedule as of writing this, so give me a sub :)

## State of Development

The state of development is accurately reflected in the number of passing tests in the [integration tests](./tests).

These tests are based around blarggs test roms for now. The emulator provides functionality to tap into 
the serial port and read bytes being pushed into it. 

The integration suite runs the emulator against test roms, and monitors the serial port until either 
"passed" or "failed" is emitted, or the test times out.

