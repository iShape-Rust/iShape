use i_float::int::number::int::IntNumber;
use i_shape::flat::buffer::{FlatContoursBuffer, FlatShapesBuffer};
use i_shape::int::IntPoint;
use i_shape::int::safe_range::IntSafeRange;
use i_shape::int::shape::{IntContour, IntShape, IntShapes};

fn check_boundaries<I: IntNumber>() {
    let limit = I::ONE << (I::BITS - 2);
    let inner = limit - I::ONE;
    let contour: IntContour<I> = vec![IntPoint::new(-inner, -inner), IntPoint::new(inner, inner)];
    assert!(contour.is_in_safe_range());
    assert!(contour.as_slice().is_in_safe_range());
    let shape: IntShape<I> = vec![vec![], contour.clone()];
    assert!(shape.is_in_safe_range());
    let shapes: IntShapes<I> = vec![vec![], shape.clone()];
    assert!(shapes.is_in_safe_range());

    for value in [-limit, limit, I::MIN, I::MAX] {
        for point in [IntPoint::new(value, I::ZERO), IntPoint::new(I::ZERO, value)] {
            let mut invalid = contour.clone();
            invalid.push(point);
            assert!(!invalid.is_in_safe_range());
            let invalid_shape = vec![contour.clone(), vec![], invalid];
            assert!(!invalid_shape.is_in_safe_range());
            assert!(!invalid_shape.as_slice().is_in_safe_range());
            let invalid_shapes = vec![shape.clone(), vec![], invalid_shape];
            assert!(!invalid_shapes.is_in_safe_range());
            assert!(!invalid_shapes.as_slice().is_in_safe_range());
        }
    }
}

#[test]
fn boundaries_for_all_integer_types() {
    check_boundaries::<i16>();
    check_boundaries::<i32>();
    check_boundaries::<i64>();
}

#[test]
fn empty_resources_are_in_range() {
    assert!(IntContour::<i32>::new().is_in_safe_range());
    assert!(IntShape::<i32>::new().is_in_safe_range());
    assert!(IntShapes::<i32>::new().is_in_safe_range());
    let shapes: IntShapes<i32> = vec![vec![], vec![vec![], vec![]]];
    assert!(shapes.is_in_safe_range());
}

#[test]
fn flat_buffers_check_only_referenced_points() {
    let mut contours = FlatContoursBuffer {
        points: vec![IntPoint::new(0_i32, 0), IntPoint::new(i32::MAX, 0)],
        ranges: vec![0..1, 0..0],
    };
    assert!(contours.is_in_safe_range());
    contours.ranges.push(1..2);
    assert!(!contours.is_in_safe_range());

    let shapes = FlatShapesBuffer {
        points: contours.points,
        contour_ranges: contours.ranges,
        shape_ranges: vec![],
    };
    assert!(!shapes.is_in_safe_range());
}
