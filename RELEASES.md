## Version 0.3.0 (02/05/2026)

* Added ability to play drones
- Updated `cpal` to version 0.17.1 and added `anyhow` for error handling.

- Enhanced user input handling with a dedicated thread for commands.

- Introduced `PitchPlayer` for audio playback of notes.

- Refactored pitch detection and note representation for improved clarity and functionality.

- Updated UI to reflect changes in pitch detection and user input.

- Added tests for note parsing and validation.

## Version 0.2.0 (11/28/2025)

- Add arrow if a note is out of tune, indicating if the user needs to go sharper
  or flatter.
- Changed all the sharp and flat notes so they contains both
- Minor UI changes, now the program will print "Listening for a note..." instead
  of nothing when it first starts up

## Version 0.1.0 (11/27/2025)

- Full functionality of a tuner
- Able to print out the pitch you are currently playing
- Able to print out the note you are currently playing
- Able to print out the cents you are sharp or flat by
- Able to use colors to help user understand if they are playing in tune or if
  they need to go sharper or flatter.
