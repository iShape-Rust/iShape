use crate::source::int::resource::IntShapeResource;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

/// Checks the coordinate range of points in an integer shape resource.
pub trait IntSafeRange<I: IntNumber> {
    /// Returns whether every visited point satisfies [`IntPoint::is_in_safe_range`].
    ///
    /// Each coordinate must be strictly between `-2^(I::BITS - 2)` and
    /// `2^(I::BITS - 2)`. Empty resources and empty contours return `true`.
    /// The check stops at the first point outside the range without allocating.
    ///
    /// This checks only coordinate bounds. It does not validate geometry or
    /// guarantee that an accumulated area fits in the wide integer type.
    /// Points are visited through [`IntShapeResource::iter_paths`]; any traversal
    /// preconditions of the resource still apply.
    ///
    /// ```
    /// use i_shape::int::{IntPoint, safe_range::IntSafeRange};
    ///
    /// let contour = vec![IntPoint::new(0_i32, 0), IntPoint::new(10, 20)];
    /// assert!(contour.is_in_safe_range());
    /// let shape = vec![contour];
    /// assert!(shape.is_in_safe_range());
    /// let shapes = vec![shape];
    /// assert!(shapes.is_in_safe_range());
    /// ```
    fn is_in_safe_range(&self) -> bool;
}

impl<I: IntNumber, R: IntShapeResource<I> + ?Sized> IntSafeRange<I> for R {
    #[inline]
    fn is_in_safe_range(&self) -> bool {
        self.iter_paths()
            .all(|path| path.iter().all(IntPoint::is_in_safe_range))
    }
}
