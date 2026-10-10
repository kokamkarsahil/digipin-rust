# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Validated, allocation-free `Digipin` value type with canonical string access,
  parsing, ordering, hashing, cell centers, and public `CellBounds`.
- `Copy` coordinates, a `const Coordinates::new` constructor, and a
  `Coordinates::encode` convenience method.
- Explicit `Digipin::parse_display` and allocation-free `display_grouped`
  formatting for the official `XXX XXXX XXX` layout.
- Optional Serde support for validated canonical DIGIPIN strings.
- CI for formatting, linting, documentation, default and Serde tests, Rust 1.85,
  and bare-metal `no_std` builds.
- Migration guide, current specification references, and typed API benchmarks.
- Regression coverage for non-finite coordinates, malformed codes, exact grid
  boundaries, official vectors, and Serde validation.

### Changed

- **Breaking:** replace `get_digipin` and `get_coordinates_from_digipin` with
  `encode` and `decode`; encoding returns `Digipin` instead of `String`.
- **Breaking:** strict parsing accepts raw ten-character ASCII codes only,
  normalizes lowercase, and rejects separators and whitespace. Legacy hyphenated
  strings require an explicit conversion described in `MIGRATION.md`.
- **Breaking:** shared grid boundaries belong to the north or east cell, in
  accordance with the May 2026 India Post technical document.
- Support `no_std` with no default runtime dependencies or allocator requirement;
  optional Serde support also remains compatible with `no_std`.
- Set the minimum supported Rust version to 1.85 for edition 2024.

### Fixed

- Document cell-center quantization, precise cell dimensions, and coverage edges.
- Correct installation versions, location examples, and official display grouping.

## [0.0.5](https://github.com/kokamkarsahil/digipin-rust/compare/v0.0.4...v0.0.5) - 2026-06-12

### Fixed

- apply CodeRabbit auto-fixes
- dependabot
- README.md

### Other

- add walltime benchmark build target to codspeed workflow
- upgrade CodSpeed action to v4 and configure execution mode
- update project edition to 2024 in Cargo.toml
- 📝 Add docstrings to `improve`
- optimize DIGIPIN encoding and decoding performance using precomputed constants and byte-level operations
- *(deps)* bump serde_json in the all-cargo-dependencies group
- Add groups for all cargo dependencies in dependabot config

## [0.0.4](https://github.com/kokamkarsahil/digipin-rust/compare/v0.0.3...v0.0.4) - 2025-10-10

### Added

- update deps
- modular and remove bounds
- dependabot

### Other

- The `Coordinates::new` function was an unnecessary abstraction, as the struct's fields are public. This commit removes the function and updates all call sites to use direct struct instantiation, making the code more concise and idiomatic.
- *(deps)* bump serde_json from 1.0.142 to 1.0.143
- Remove `get_bounds_from_digipin`

## [0.0.3](https://github.com/kokamkarsahil/digipin-rust/compare/v0.0.2...v0.0.3) - 2025-08-04

### Added

- optimse code
- add benchmark

### Other

- Create release-plz.yml
- Create bench.yml
