# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Fixed

### Fixed

- Fixed cursor position logic when transitioning between SSID and password inputs for hidden networks.
- Fixed an issue where the cursor would "ghost" or jump to the right when opening the password popup by resetting the cursor position on initialization.
- Fixed the re-rendering issue of the cursor by removing it from the render method and placing it to the draw method.
- Fixed the issue of pop-up for pressing 'd' key in the unsaved connection from the main list.
- Fixed the issue with the `esc` key not closing the ssid input pop-up by changing the flag that it was checking for.

### Changed

- Added a flag to indicate whether the app is scanning the networks or not.
- Changed the type of list that was send to the scan_networks function from main App `struct` to be `RwLock` wrapped type rather than Arc wrapped type.

### Changed

- Changed the type of list that was send to the scan_networks function from main App `struct` to be `RwLock` wrapped type rather than Arc wrapped type.
- Changed the render method to render the scanning status with small animation in accordance with the scanning flag rather then the `.try_lock()` method that was used earlier.

## [1.1.0] - 2025-01-01

### Added

- Listener for disconnecting the current connection
- Helper function to disconnect from the current connection
- Utility function that will actually disconnect from the current connection

### Changed

- Changed the Flag to be in App level and added few more flags there

### Fixed

- Fixed the issue in the saved-connection list where when we try to delete a connection, it was doing the deletion from the index of the main list instead of the saved connection list.
- Fixed the refresh issue of the list after the deletion of a saved connection.
- Fixed the out of bound issue when the list gets refreshed after deletion.

## [1.0.0] - 2025-12-30

### Added

- documentation and comments
- help menu
- list for saved connections,
- delete connection confirmation pop-up,
- listeners for all the pop-ups/menus

### Changed

- code refactoring and cleanup
- force full modularization of codebase

## 0.1.0 - 2025-12-21 (init date)

### Added

- Initial project structure
- Working state with some feat like network scanning

[Unreleased]: https://github.com/santoshxshrestha/nmtui/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/santoshxshrestha/nmtui/releases/tag/v1.1.0
[1.0.0]: https://github.com/santoshxshrestha/nmtui/releases/tag/v1.0.0
