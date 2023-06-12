# Changelog
Notable changes to **DMNTK** project are documented in this file.

This format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

See [Issues](https://github.com/dmntk/dmntk.rs/issues).

## [v0.2.0 (2nd beta)] - 2023-06-12

### Added
- Importing models, see [issue #35](https://github.com/dmntk/dmntk.rs/issues/35).
- Added color switches in several commands, see [issue #24](https://github.com/dmntk/dmntk.rs/issues/24).
- Refactored multiple components.
- Improved code coverage.
- Added more tests.
 
### Changed
- Refactored parser tests, see [issue #8](https://github.com/dmntk/dmntk.rs/issues/8).
- Invocable deployment path now contains the namespace in RDNN format and the invocable name (or identifier).

## [v0.1.2 (fixes to 1st beta)] - 2023-05-16

- Fixed coercion while reading lists containing structures with sparse null values.
- Fixed parsing error for multiple consecutive comments.
- Refactored (optimized) collection of values.
- Improved exporting DMN model to HTML format (alpha stage).
- Added exporting decision tables to HTML format (alpha stage).
- Fixed issues reported by users.
- Code lines coverage over 95%.
- Code refactoring. 

## [v0.1.1 (fixes to 1st beta)] - 2023-02-10

- Fixed minor issues with path names.
- Improved code coverage.
- Added more tests.
- Code lines coverage over 90%.

## [v0.1.0 (1st beta)] - 2023-01-20

- Implemented lambdas.
- Fixed non-conformant functionality tested by flight rebooking test suite.
- Refactored **dmntk-feel-parser** component.
- Improved code coverage.
- Implemented more compatibility tests (99,9% passes).
- Implemented external Java functions invocation with mocked tests.

## [v0.0.54 (3rd alpha)] - 2023-01-10

### Added
- Implemented handling user defined functions.
- Fixed minor errors.
- Implemented more compatibility tests (99,3% passes).
- Added tests to improve code coverage.

## [v0.0.53 (2nd alpha)] - 2023-01-05

### Added
- Refactored error handling.
- Fixed minor errors.
- Implemented lacking compatibility tests (99,0% passes).
- Improved code coverage.

## [v0.0.52 (1st alpha)] - 2022-12-16

### Added
- Implemented lacking built-in functions.
- Prepared builds for multiple platforms.
- Implemented command-line command for recognizing decision tables. 

## [v0.0.46 (pre-alpha)] - 2022-01-31

### Added
- Performance benchmarks.
- Test files for manual tests.
- CLI switch for displaying only summary after tests using test files.

### Changed
- Refactored CLI. 

## [v0.0.45 (pre-alpha)] - 2022-01-13

### Added
- CLI subcommand for saving examples.
- Implemented CLI subcommands: **tdt**, **edt**, **tdm**, **edm**, **pdm**, **tfe**.
- Added contributor covenant.

### Changed
- Removed **dmntk_recognizer** dependency from **dmntk_model_evaluator**. 

## [v0.0.44 (pre-alpha)] - 2021-12-29

### Released
- Released the first **pre-alpha** version of **DMNTK**.