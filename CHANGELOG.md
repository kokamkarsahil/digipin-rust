# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
