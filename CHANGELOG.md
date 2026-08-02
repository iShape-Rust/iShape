# Changelog

All notable changes to this project are documented in this file.

## [4.0.0] - Unreleased

### Changed

- Updated `i_float` to 4.0 and migrated integer widening calls to `IntNumber::to_wide`.
- Declared Rust 1.85 as the minimum supported Rust version.
- Reworked collection macros to allocate their output directly from iterators.
- Renamed `ContourExtension::contains` to `contains_point` to avoid a collision with the slice method of the same name.
- Removed the `util::reserve::Reserve` extension trait in favor of `Vec::clear` and `Vec::reserve`.
- Added `PartialEq` and `Eq` implementations for flat buffer types.
- Preserved empty contours when iterating a floating-point flat buffer as a `ShapeResource`.

### Fixed

- Prevented `FloatFlatContoursBuffer::simplify_contour` from quantizing an unchanged buffer.
- Fixed `ShapeResource` iterator counts after partial consumption.
- Made integer containment handle empty contours without panicking.
- Removed integer-division rounding from point-in-contour tests by comparing
  edge topology with a wide cross product.
- Removed unnecessary unchecked indexing from shape resource iterators.
- Corrected floating-point flat-buffer capacity allocation.

[4.0.0]: https://github.com/iShape-Rust/i_shape/compare/3.0.0...HEAD
