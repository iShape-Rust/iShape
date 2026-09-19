use i_shape::flat::buffer::{FlatContoursBuffer, FlatShapesBuffer};
use i_shape::flat::float::{FloatFlatContoursBuffer, FloatFlatShapesBuffer};
use i_shape::int::IntPoint;
use i_shape::source::float::resource::ShapeResource;
use i_shape::source::int::resource::IntShapeResource;

// Using a generic function (rather than method autoderef) also checks forwarding
// implementations for arrays, Vecs, and references.
fn int_paths<I: i_float::int::number::int::IntNumber, R: IntShapeResource<I> + ?Sized>(
    resource: &R,
) -> Vec<&[IntPoint<I>]> {
    resource.iter_paths().collect()
}

fn float_paths<P: i_float::float::compatible::FloatPointCompatible, R: ShapeResource<P> + ?Sized>(
    resource: &R,
) -> Vec<&[P]> {
    resource.iter_paths().collect()
}

macro_rules! container_contract {
    ($name:ident, $collect:ident, $points:expr) => {
        #[test]
        fn $name() {
            let points = $points;
            let path = points.to_vec();
            let expected = vec![points.as_slice()];
            assert_eq!($collect(&points), expected);
            assert_eq!($collect(points.as_slice()), expected);
            assert_eq!($collect(&path), expected);
            assert_eq!($collect(&&path), expected);
            assert_eq!($collect(&&&path), expected);
            let mut owned = path.clone();
            assert_eq!($collect(&&mut owned), expected);

            let shape = vec![path.clone(), vec![], path.clone()];
            let expected = vec![points.as_slice(), &[], points.as_slice()];
            assert_eq!($collect(&shape), expected);
            assert_eq!($collect(shape.as_slice()), expected);
            assert_eq!($collect(&[path.clone(), vec![], path.clone()]), expected);
            assert_eq!($collect(&&shape[..]), expected);

            let shapes = vec![vec![], shape.clone(), vec![], vec![vec![]], vec![]];
            let expected = vec![points.as_slice(), &[], points.as_slice(), &[]];
            assert_eq!($collect(&shapes), expected);
            assert_eq!($collect(shapes.as_slice()), expected);
            assert_eq!($collect(&[vec![], shape, vec![vec![]]]), expected);
            assert_eq!($collect(&&shapes[..]), expected);

            let borrowed = [points.as_slice(), &[], &points[1..]];
            assert_eq!($collect(&borrowed), borrowed);
            assert_eq!($collect(borrowed.as_slice()), borrowed);
            assert_eq!($collect(&borrowed.to_vec()), borrowed);
            let slices = $collect(&borrowed);
            assert_eq!(slices[0].as_ptr(), points.as_ptr());
            assert_eq!(slices[2].as_ptr(), points[1..].as_ptr());
            // Independent traversals borrow the original storage as well.
            assert_eq!($collect(&path)[0].as_ptr(), path.as_ptr());
            assert_eq!($collect(&path)[0].as_ptr(), path.as_ptr());

            let empty_path = &points[..0];
            assert_eq!($collect(empty_path), vec![empty_path]);
            let empty_shape = [path.clone(); 0];
            assert!($collect(&empty_shape).is_empty());
            let empty_shapes = [vec![path]; 0];
            assert!($collect(&empty_shapes).is_empty());
        }
    };
}

container_contract!(
    integer_i16,
    int_paths,
    [IntPoint::new(7_i16, 1), IntPoint::new(2, -3)]
);
container_contract!(
    integer_i32,
    int_paths,
    [IntPoint::new(7_i32, 1), IntPoint::new(2, -3)]
);
container_contract!(
    integer_i64,
    int_paths,
    [IntPoint::new((1_i64 << 60) + 1, 1), IntPoint::new(2, -3)]
);
container_contract!(float_f32, float_paths, [[7_f32, 1.], [2., -3.]]);
container_contract!(float_f64, float_paths, [[7_f64, 1.], [2., -3.]]);

#[test]
fn both_traits_in_scope_preserve_method_inference_and_remaining_count() {
    let shape = vec![vec![IntPoint::new(1_i32, 2)], vec![], vec![IntPoint::new(3, 4)]];
    let shapes = vec![vec![], shape, vec![]];
    let mut iter = shapes.iter_paths();
    assert_eq!(iter.next().unwrap(), &[IntPoint::new(1, 2)]);
    assert_eq!(iter.count(), 2);
    let mut iter = shapes.iter_paths();
    while iter.next().is_some() {}
    assert!(iter.next().is_none());
    assert_eq!(iter.count(), 0);

    let floats = [vec![[1_f64, 2.]], vec![]];
    assert_eq!(floats.iter_paths().count(), 2);
}

#[test]
fn flat_resources_preserve_storage_order_empty_ranges_and_borrows() {
    let points = vec![IntPoint::new(1_i32, 2), IntPoint::new(3, 4)];
    let ranges = vec![1..2, 0..0, 0..2];
    let contours = FlatContoursBuffer {
        points: points.clone(),
        ranges: ranges.clone(),
    };
    let shapes = FlatShapesBuffer {
        points,
        contour_ranges: ranges,
        shape_ranges: vec![],
    };
    let expected = vec![&shapes.points[1..2], &shapes.points[0..0], &shapes.points[..]];
    assert_eq!(int_paths(&contours), expected);
    assert_eq!(int_paths(&shapes), expected);
    assert_eq!(int_paths(&shapes)[0].as_ptr(), shapes.points[1..].as_ptr());
    assert_eq!(int_paths(&contours)[2].as_ptr(), contours.points.as_ptr());
    let mut iter = contours.iter_paths();
    assert_eq!(iter.len(), 3);
    iter.next();
    assert_eq!(iter.count(), 2);

    let contours = FloatFlatContoursBuffer {
        points: vec![[1_f32, 2.], [3., 4.]],
        ranges: vec![1..2, 0..0, 0..2],
    };
    let shapes = FloatFlatShapesBuffer {
        points: contours.points.clone(),
        contour_ranges: contours.ranges.clone(),
        shape_ranges: vec![],
    };
    assert_eq!(float_paths(&contours), float_paths(&shapes));
    assert_eq!(float_paths(&contours)[2].as_ptr(), contours.points.as_ptr());
}

#[test]
#[should_panic]
fn integer_flat_contours_reject_out_of_bounds_ranges() {
    let buffer = FlatContoursBuffer::<i32> {
        points: vec![],
        ranges: vec![core::ops::Range { start: 0, end: 1 }],
    };
    buffer.iter_paths().next();
}

#[test]
#[should_panic]
fn integer_flat_shapes_reject_reversed_ranges() {
    let buffer = FlatShapesBuffer::<i32> {
        points: vec![IntPoint::new(0, 0)],
        contour_ranges: vec![core::ops::Range { start: 1, end: 0 }],
        shape_ranges: vec![],
    };
    buffer.iter_paths().next();
}

// A downstream resource can name its iterator and borrow data with a lifetime
// shorter than 'static. Holding one returned path does not block further reads.
struct BorrowedPath<'p>(&'p [IntPoint<i32>]);

impl IntShapeResource<i32> for BorrowedPath<'_> {
    type ResourceIter<'a>
        = core::iter::Once<&'a [IntPoint<i32>]>
    where
        Self: 'a;

    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        core::iter::once(self.0)
    }
}

#[test]
fn downstream_resource_borrows_local_storage() {
    let points = [IntPoint::new(1, 2)];
    let resource = BorrowedPath(&points);
    let first = int_paths(&resource)[0];
    assert_eq!(int_paths(&&resource), vec![first]);
    assert_eq!(first.as_ptr(), points.as_ptr());
}

#[test]
fn float_resource_conversion_flattens_shapes_and_preserves_empty_paths() {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;
    use i_shape::float::adapter::{PathToInt, ResourceToIntIter};

    let adapter = FloatPointAdapter::<[f64; 2], i32>::with_scale(
        FloatRect::new(-10.0, 10.0, -10.0, 10.0).unwrap(),
        2.0,
    );
    let path = vec![[1.5, -2.0], [-3.0, 4.5]];
    let expected = vec![IntPoint::new(3, -4), IntPoint::new(-6, 9)];
    // Both conversion traits remain usable, including on unsized slices.
    assert_eq!(path.to_int(&adapter), expected);
    assert_eq!(
        path[..]
            .iter_int_paths(&adapter)
            .map(|path| path.collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        vec![expected.clone()]
    );
    let shapes = [vec![], vec![path, vec![]], vec![vec![[0.0, 0.0]]]];
    assert_eq!(
        shapes[..]
            .iter_int_paths(&adapter)
            .map(|path| path.collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        vec![expected, vec![], vec![IntPoint::new(0, 0)]]
    );
    assert!(shapes[..0].iter_int_paths(&adapter).next().is_none());
}

#[test]
fn float_resource_conversion_respects_flat_buffer_ranges() {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;
    use i_shape::float::adapter::ResourceToIntIter;

    let buffer = FloatFlatContoursBuffer {
        points: vec![[1.0_f32, 2.0], [-3.0, 4.0]],
        ranges: vec![1..2, 0..0, 0..2],
    };
    let adapter =
        FloatPointAdapter::<_, i16>::with_scale(FloatRect::new(-10.0, 10.0, -10.0, 10.0).unwrap(), 1.0);
    assert_eq!(
        buffer
            .iter_int_paths(&adapter)
            .map(|path| path.collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        vec![
            vec![IntPoint::new(-3, 4)],
            vec![],
            vec![IntPoint::new(1, 2), IntPoint::new(-3, 4)],
        ]
    );
}
