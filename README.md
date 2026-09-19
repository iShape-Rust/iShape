# iShape

[![crates.io](https://img.shields.io/crates/v/i_shape.svg)](https://crates.io/crates/i_shape)
[![docs.rs](https://docs.rs/i_shape/badge.svg)](https://docs.rs/i_shape)
[![license](https://img.shields.io/crates/l/i_shape.svg)](https://crates.io/crates/i_shape)

`i_shape` provides compact, generic data structures and utilities for 2D polygon data. It includes:

- integer and floating-point path, contour, shape, and shapes aliases;
- flat contour and shape buffers for allocation-efficient geometry pipelines;
- area, winding, convexity, simplification, and despiking helpers;
- conversion between floating-point and integer coordinates through `i_float`;
- `IntShapeResource` and `ShapeResource` interfaces for borrowing integer and floating-point contours, shapes, and flat buffers.

The crate is `no_std`, uses `alloc`, and supports `i16`, `i32`, and `i64` integer coordinates.

The `int::simple` and `float::simple` helpers remove collinear and consecutive
duplicate vertices. They do not check or resolve self-intersections;
`SimpleContour::is_simple` checks only vertex count and adjacent edge cross products.

## Installation

```toml
[dependencies]
i_shape = "4.0"
```

Enable `serde` when serialized geometry or flat buffers are required:

```toml
[dependencies]
i_shape = { version = "4.0", features = ["serde"] }
```

## Integer contours

```rust
use i_shape::int::area::Area;
use i_shape::int::path::ContourExtension;
use i_shape::int_path;

let contour = int_path![[0_i32, 0], [10, 0], [10, 10], [0, 10]];

assert_eq!(contour.area_two(), 200_i64);
assert!(!contour.is_clockwise_ordered());
assert!(contour.contains_point(i_shape::int::IntPoint::new(5, 5)));
```

Integer operations use the wide type associated with the coordinate type. See the
[`i_float` coordinate-range documentation](https://docs.rs/i_float/latest/i_float/int/point/struct.IntPoint.html)
for the arithmetic preconditions that apply to geometric predicates.

## Flat buffers

Flat buffers store points contiguously and describe contours and shapes with index ranges. This is useful when passing geometry between algorithms without rebuilding nested `Vec`s.

```rust
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::int_path;

let contour = int_path![[0_i32, 0], [4, 0], [4, 4], [0, 4]];
let mut buffer = FlatContoursBuffer::default();
buffer.add_contour(&contour);

assert_eq!(buffer.ranges, [0..4]);
assert_eq!(buffer.to_contours(), vec![contour]);
```

Floating-point equivalents are available as `FloatFlatContoursBuffer<P>` and `FloatFlatShapesBuffer<P>`, where `P` can be any `FloatPointCompatible` type such as `[f32; 2]` or `[f64; 2]`.

## Borrowed resources

Integer and floating-point resources expose the same `iter_paths()` operation:

```rust
use i_shape::int_path;
use i_shape::source::int::resource::IntShapeResource;
use i_shape::source::float::resource::ShapeResource;

let integers = int_path![[0_i32, 0], [4, 0], [4, 4]];
let floats = vec![vec![[0.0_f64, 0.0], [4.0, 0.0], [4.0, 4.0]]];

assert_eq!(integers.iter_paths().next(), Some(integers.as_slice()));
assert_eq!(floats.iter_paths().count(), 1);

// Borrow multiple contours without copying points.
let borrowed = [integers.as_slice(), &integers[1..]];
assert_eq!(borrowed.iter_paths().count(), 2);
```

Both interfaces support a single path, a collection of paths, and a collection
of shapes, using slices, vectors, or outer fixed-size arrays. Collections of
borrowed path slices, shared/mutable references to resources, and the corresponding
flat buffers are also supported. Iteration borrows the original points, preserves
path and vertex order, includes empty paths, and can be restarted without allocating.

Shape grouping is flattened: `FlatShapesBuffer` and `FloatFlatShapesBuffer` visit
`contour_ranges` independently of `shape_ranges`. These interfaces do not impose
winding, closure, or geometric validity requirements; consuming operations define
their own requirements. Invalid integer flat-buffer contour ranges panic when
visited. Floating-point flat buffers retain their existing behavior of skipping
invalid ranges.

**Breaking migration:** replace `source::resource::ShapeResource` imports with
`source::float::resource::ShapeResource`. The other previous `source` modules
(`contour`, `shape`, `shapes`, and `buffer`) have also moved under `source::float`.
There are no compatibility re-exports at the old paths. Integer consumers use
`source::int::resource::IntShapeResource`.

## License

Licensed under the MIT License. See the [license file](https://github.com/iShape-Rust/i_shape/blob/main/LICENSE).
