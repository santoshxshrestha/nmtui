# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Fixed

- Fixed the rerendering issue of the cursor by removing it from the render method and placing it to the draw method.

## [1.1.0] - 2025-01-01

### Added

- Listner for disconnecting the current connection
- Helper function to disconnect from the current connection
- Utility function that will actually disconnect from the current connection

### Changed

- Changed the Flag to be in App level and added few more flags there

### Fixed

- Fixed the issue in the saved-connection list where when we try to delete a connection, it was doing the deletion from the index of the main list instead of the saved connection list.
- Fixed the refresh issue of the list after the deletion of a saved connection.
- Fixed the outo of bound issue when the list gets refreshed after deletion.

## [1.0.0] - 2025-12-30

### Added

- documentation and comments
- help menu
- list for saved connections,
- delete connection confirmation popup,
- listeners for all the popups/menus

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
