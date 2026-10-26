# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to Semantic Versioning ([SemVer]).

## [Unreleased]

### Added

- Export book as epub and create file on filesystem.
- Import metadata from `.toml` file to epub

## [0.1.0] - 2026-10-16

### Added

- New flag for format with `epub` as default value.
- New flag for output path using `book_path` flag as default value.

### Changed

- Split into two crates: core and cli.

## [0.0.1] - 2026-10-03

### Added

- Bootstrap project with basic CLI structure.
- Read `book.toml` file and show book title.

[Keep a Changelog]: https://keepachangelog.com/en/0.3.0/
[SemVer]: https://semver.org/spec/v2.0.0.html
[0.1.0]: https://github.com/jlucfarias/toth/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/jlucfarias/toth/releases/tag/v0.0.1
