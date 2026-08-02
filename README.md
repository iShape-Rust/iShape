# iShape

[![crates.io](https://img.shields.io/crates/v/i_shape.svg)](https://crates.io/crates/i_shape)
[![docs.rs](https://docs.rs/i_shape/badge.svg)](https://docs.rs/i_shape)
[![license](https://img.shields.io/crates/l/i_shape.svg)](https://crates.io/crates/i_shape)

`i_shape` provides compact, generic data structures and utilities for 2D polygon data. It includes:

- integer and floating-point path, contour, shape, and shapes aliases;
- flat contour and shape buffers for allocation-efficient geometry pipelines;
- area, winding, convexity, simplification, despiking, and deduplication helpers;
- conversion between floating-point and integer coordinates through `i_float`;
- a common `ShapeResource` interface for accepting contours, shapes, and flat buffers.

The crate is `no_std`, uses `alloc`, and supports `i16`, `i32`, and `i64` integer coordinates.

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

## License

Licensed under the MIT License. See the [license file](https://github.com/iShape-Rust/i_shape/blob/main/LICENSE).
