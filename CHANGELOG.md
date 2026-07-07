# Changelog

<!-- Ref: https://keepachangelog.com/ -->

## [0.6.0] - 2026-04-12

### Added

- `as_str`, `as_bytes` and `len` are now `const fn`. ([#3](https://github.com/ryota2357/lean_string/pull/3) by [@lperlaki](https://github.com/lperlaki))
- `split_off` method.
- `repeat` method.

## [0.5.3] - 2026-03-18

### Changed

- Depend on `serde_core` instead of `serde` for the `serde` feature.

### Fixed

- Use-after-free that could occur under clone-on-write in `reserve`, `ensure_modifiable` and `truncate_unchecked` by switching to a read-only uniqueness check.

## [0.5.2] - 2026-03-12

### Changed

- Various performance improvements: specialized `Write` for static `&str`, faster `len()`, and `#[inline]` on `serde` and `Drop` implementations.

## [0.5.1] - 2025-08-24

### Fixed

- Memory leak in `shrink_to()`.

## [0.5.0] - 2025-05-16

### Added

- `truncate()` method.

### Changed

- Migrated to the Rust 2024 edition.

### Removed

- **Breaking:** the `last_byte` feature.

## [0.4.0] - 2025-01-21

### Changed

- `pop()` no longer clones the buffer, improving performance.

## [0.3.0] - 2024-12-27

### Added

- Support for 32-bit architectures.

### Fixed

- Panic on reference count overflow instead of risking unsound behavior.

## [0.2.0] - 2024-12-22

### Changed

- Performance improvements via specialization for `ToLeanString` and `From<char>`, plus `#[inline]` on several methods.

## [0.1.0] - 2024-12-20

Initial release.

### Added

- `LeanString`: a compact, clone-on-write string with small-string optimization.
- Growable string API: `push`, `push_str`, `pop`, `insert*`, `remove`, `retain`, `shrink_to*`, `with_capacity` and fallible `reserve`.
- Constructors `from_utf8` / `from_utf16` and the `ToLeanString` trait.
- `no_std` support.
- Optional `serde` and `arbitrary` support.
- `Send` and `Sync` implementations, verified with `loom` and `miri`.

[0.6.0]: https://github.com/ryota2357/lean_string/compare/v0.5.3...v0.6.0
[0.5.3]: https://github.com/ryota2357/lean_string/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/ryota2357/lean_string/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/ryota2357/lean_string/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/ryota2357/lean_string/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/ryota2357/lean_string/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ryota2357/lean_string/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ryota2357/lean_string/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ryota2357/lean_string/releases/tag/v0.1.0
