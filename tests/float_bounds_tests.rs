use i_float::float::number::FloatNumber;
use i_float::float::rect::{FloatRect, FloatRectError};
use i_shape::flat::buffer::{FlatContoursBuffer, FlatShapesBuffer};
use i_shape::float::rect::RectInit;

fn check_rects<T: FloatNumber>() {
    let empty: Vec<[T; 2]> = vec![];
    assert!(FloatRect::with_path(&empty).unwrap().is_none());
    assert!(FloatRect::with_paths(&[empty.clone()]).unwrap().is_none());
    assert!(
        FloatRect::with_list_of_paths(&[vec![], vec![empty.clone()]])
            .unwrap()
            .is_none()
    );

    let limit = T::MAX_COORDINATE;
    let path = vec![[-limit, T::ZERO], [T::ZERO, limit], [limit, -limit]];
    let paths = vec![empty.clone(), path.clone(), empty.clone()];
    let list = vec![vec![], paths.clone(), vec![empty.clone()]];
    for result in [
        FloatRect::with_path(&path),
        FloatRect::with_paths(&paths),
        FloatRect::with_list_of_paths(&list),
    ] {
        let rect = result.unwrap().unwrap();
        assert!(rect.min_x == -limit && rect.max_x == limit);
        assert!(rect.min_y == -limit && rect.max_y == limit);
    }

    for invalid in [
        T::from_float(f64::NAN),
        T::from_float(f64::INFINITY),
        T::from_float(f64::NEG_INFINITY),
        limit + limit,
        -limit - limit,
    ] {
        // The invalid point follows valid input, including across empty paths.
        let path = vec![[T::ZERO, T::ZERO], [invalid, T::ZERO]];
        let paths = vec![vec![path[0]], empty.clone(), vec![path[1]]];
        let list = vec![vec![], paths.clone()];
        for result in [
            FloatRect::with_path(&path),
            FloatRect::with_paths(&paths),
            FloatRect::with_list_of_paths(&list),
        ] {
            assert!(matches!(result, Err(FloatRectError::CoordinatesOutOfRange)));
        }
    }
}

#[test]
fn rects_distinguish_empty_valid_and_invalid_input() {
    check_rects::<f32>();
    check_rects::<f64>();
}

#[test]
fn empty_resource_clears_both_buffers_and_returns_zero_bounds() {
    let path = [[1.0_f64, 2.0], [3.0, 4.0]];
    let mut contours = FlatContoursBuffer::<i32>::default();
    let mut shapes = FlatShapesBuffer::<i32>::default();
    contours.set_with_resource(&path).unwrap();
    shapes.set_with_resource(&path).unwrap();

    let empty = vec![Vec::<[f64; 2]>::new()];
    for adapter in [
        contours.set_with_resource(&empty).unwrap(),
        shapes.set_with_resource(&empty).unwrap(),
    ] {
        let rect = adapter.rect();
        assert_eq!(
            (rect.min_x, rect.max_x, rect.min_y, rect.max_y),
            (0.0, 0.0, 0.0, 0.0)
        );
        assert_eq!(adapter.dir_scale(), 1.0);
    }
    assert!(contours.points.is_empty() && contours.ranges.is_empty());
    assert!(shapes.points.is_empty());
    assert!(shapes.contour_ranges.is_empty() && shapes.shape_ranges.is_empty());
}

// Debug builds reject these points at the contract assertion before constructing bounds.
#[cfg(not(debug_assertions))]
#[test]
fn invalid_bounds_return_errors_without_changing_buffers() {
    let path = [[1.0_f64, 2.0], [3.0, 4.0]];
    let mut contours = FlatContoursBuffer::<i32>::default();
    let mut shapes = FlatShapesBuffer::<i32>::default();
    contours.set_with_resource(&path).unwrap();
    shapes.set_with_resource(&path).unwrap();
    let original_contours = contours.clone();
    let original_shapes = shapes.clone();

    for invalid in [f64::INFINITY, f64::NEG_INFINITY, f64::MAX_COORDINATE * 2.0] {
        let invalid_path = [[0.0, 0.0], [invalid, 0.0]];
        assert!(matches!(
            contours.set_with_resource(&invalid_path),
            Err(FloatRectError::CoordinatesOutOfRange)
        ));
        assert!(matches!(
            shapes.set_with_resource(&invalid_path),
            Err(FloatRectError::CoordinatesOutOfRange)
        ));
        assert_eq!(contours, original_contours);
        assert_eq!(shapes, original_shapes);
    }
}

#[test]
fn resource_adapters_use_the_conservative_coordinate_budget() {
    use i_float::int::number::int::IntNumber;

    fn check<I: IntNumber>() {
        let radius = 1.999_999_f64;
        let path = [
            [-radius, -radius],
            [radius, -radius],
            [radius, radius],
            [-radius, radius],
        ];
        let mut contours = FlatContoursBuffer::<I>::default();
        let mut shapes = FlatShapesBuffer::<I>::default();
        let contour_adapter = contours.set_with_resource(&path).unwrap();
        let shape_adapter = shapes.set_with_resource(&path).unwrap();
        let limit = I::ONE << (I::BITS - 3);
        for point in contours.points.iter().chain(&shapes.points) {
            assert!(point.x >= -limit && point.x <= limit);
            assert!(point.y >= -limit && point.y <= limit);
        }
        assert_eq!(contour_adapter.dir_scale(), shape_adapter.dir_scale());
        for (original, point) in path.iter().zip(&contours.points) {
            let restored = contour_adapter.int_to_float(point);
            assert!((restored[0] - original[0]).abs() <= contour_adapter.inv_scale());
            assert!((restored[1] - original[1]).abs() <= contour_adapter.inv_scale());
        }
    }
    check::<i16>();
    check::<i32>();
    check::<i64>();
}
