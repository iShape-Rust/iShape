use crate::flat::buffer::{FlatContoursBuffer, FlatShapesBuffer};
use crate::source::float::resource::ShapeResource;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::{FloatRect, FloatRectError};
use i_float::int::number::int::IntNumber;

impl<I: IntNumber> FlatContoursBuffer<I> {
    /// Replaces the buffer, leaving it unchanged if the bounds are invalid.
    /// Points must satisfy the `i_float::float` coordinate contract; per-point checks
    /// run only in debug builds. Uses the conservative adapter coordinate budget.
    /// Empty input clears the buffer and returns a zero-bounds adapter.
    #[inline]
    pub fn set_with_resource<P, R>(&mut self, resource: &R) -> Result<FloatPointAdapter<P, I>, FloatRectError>
    where
        I: IntNumber,
        P: FloatPointCompatible,
        R: ShapeResource<P> + ?Sized,
    {
        let mut contours_count = 0;
        let mut points_count = 0;
        let mut min_x = P::Scalar::MAX;
        let mut max_x = P::Scalar::MIN;
        let mut min_y = P::Scalar::MAX;
        let mut max_y = P::Scalar::MIN;
        for contour in resource.iter_paths() {
            contours_count += 1;
            points_count += contour.len();
            for p in contour.iter() {
                debug_assert!(p.is_in_safe_range(), "FloatPoint coordinates out of range");
                min_x = min_x.min(p.x());
                max_x = max_x.max(p.x());
                min_y = min_y.min(p.y());
                max_y = max_y.max(p.y());
            }
        }

        if points_count == 0 {
            self.clear_and_reserve(points_count, contours_count);
            return Ok(FloatPointAdapter::new_conservative(FloatRect::zero()));
        }

        let rect = FloatRect::new(min_x, max_x, min_y, max_y)?;
        let adapter = FloatPointAdapter::new_conservative(rect);
        self.clear_and_reserve(points_count, contours_count);

        let mut offset = 0;
        for contour in resource.iter_paths() {
            for p in contour.iter() {
                self.points.push(adapter.float_to_int(p));
            }
            let contour_len = contour.len();
            self.ranges.push(offset..offset + contour_len);
            offset += contour_len;
        }

        Ok(adapter)
    }
}

impl<I: IntNumber> FlatShapesBuffer<I> {
    /// Replaces the buffer, leaving it unchanged if the bounds are invalid.
    /// Points must satisfy the `i_float::float` coordinate contract; per-point checks
    /// run only in debug builds. Uses the conservative adapter coordinate budget.
    /// Empty input clears the buffer and returns a zero-bounds adapter.
    #[inline]
    pub fn set_with_resource<P, R>(&mut self, resource: &R) -> Result<FloatPointAdapter<P, I>, FloatRectError>
    where
        P: FloatPointCompatible,
        R: ShapeResource<P> + ?Sized,
    {
        let mut contours_count = 0;
        let mut points_count = 0;
        let mut min_x = P::Scalar::MAX;
        let mut max_x = P::Scalar::MIN;
        let mut min_y = P::Scalar::MAX;
        let mut max_y = P::Scalar::MIN;
        for contour in resource.iter_paths() {
            contours_count += 1;
            points_count += contour.len();
            for p in contour.iter() {
                debug_assert!(p.is_in_safe_range(), "FloatPoint coordinates out of range");
                min_x = min_x.min(p.x());
                max_x = max_x.max(p.x());
                min_y = min_y.min(p.y());
                max_y = max_y.max(p.y());
            }
        }

        if points_count == 0 {
            self.clear_and_reserve(points_count, contours_count, usize::from(contours_count > 0));
            return Ok(FloatPointAdapter::new_conservative(FloatRect::zero()));
        }

        let rect = FloatRect::new(min_x, max_x, min_y, max_y)?;
        let adapter = FloatPointAdapter::new_conservative(rect);
        self.clear_and_reserve(points_count, contours_count, usize::from(contours_count > 0));

        let mut offset = 0;
        for contour in resource.iter_paths() {
            for p in contour.iter() {
                self.points.push(adapter.float_to_int(p));
            }
            let contour_len = contour.len();
            self.contour_ranges.push(offset..offset + contour_len);
            offset += contour_len;
        }

        self.shape_ranges.push(0..contours_count);

        Ok(adapter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_shapes_buffer_set_with_resource() {
        let shape: Vec<Vec<[f64; 2]>> = vec![
            vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0]],
            vec![[1.0, 1.0], [1.5, 1.0], [1.5, 1.5]],
        ];

        let mut buffer = FlatShapesBuffer::<i32>::default();
        let adapter = buffer.set_with_resource(&shape).unwrap();
        let restored: Vec<Vec<[f64; 2]>> = buffer
            .to_shapes()
            .into_iter()
            .next()
            .unwrap()
            .iter()
            .map(|contour| contour.iter().map(|point| adapter.int_to_float(point)).collect())
            .collect();

        assert_eq!(buffer.shape_ranges, vec![0..2]);
        assert_eq!(buffer.contour_ranges, vec![0..3, 3..6]);
        assert_eq!(restored, shape);
    }
}
