# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate
### Fixed
- fix `bytelevel::Product` append operation
- count padding bytes in field size
- handling of `PNil→[{Bytes,Array}<_, U0>|...]`

## [0.2.2] - 2020-02-07
### Added
- `#[no_std]` compatible.
- [`GenericArray`](https://crates.io/crates/generic-array) support.
- Expose `layout` module providing some of typic's type-level information about
  types (namely: size and minimum alignment).

## [0.2.1] - 2020-02-06
### Fixed
- Removed unused, nightly-only feature.

## [0.2.0] - 2020-02-06
### Breaking Changes
- Everything. This is a complete rewrite.

## 0.1.0 - 2019-12-28
### Added
- Initial, prototype release.

<!-- next-url -->
[Unreleased]: https://github.com/jswrenn/typic/compare/{{tag_name}}...HEAD
[0.2.2]: https://github.com/typic/compare/typic-v0.2.1...{{tag_name}}
[0.2.1]: https://github.com/jswrenn/typic/compare/typic-v0.2.0...typic-v0.2.1
[0.2.0]: https://github.com/jswrenn/typic/releases/tag/typic-v0.2.0
