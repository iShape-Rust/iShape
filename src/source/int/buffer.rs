use super::resource::IntShapeResource;
use crate::flat::buffer::{FlatContoursBuffer, FlatShapesBuffer};
use core::{iter::FusedIterator, ops::Range, slice};
use i_float::int::{number::int::IntNumber, point::IntPoint};

/// Borrowed paths addressed by a flat buffer's contour ranges.
///
/// # Panics
///
/// Advancing the iterator panics if the next range is reversed or out of bounds.
pub struct IntBufferResourceIterator<'a, I: IntNumber> {
    points: &'a [IntPoint<I>],
    ranges: slice::Iter<'a, Range<usize>>,
}

impl<'a, I: IntNumber> Iterator for IntBufferResourceIterator<'a, I> {
    type Item = &'a [IntPoint<I>];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.ranges.next().map(|range| &self.points[range.clone()])
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ranges.size_hint()
    }
}

impl<I: IntNumber> ExactSizeIterator for IntBufferResourceIterator<'_, I> {}
impl<I: IntNumber> FusedIterator for IntBufferResourceIterator<'_, I> {}

/// Visits ranges in storage order, including empty ranges.
/// Invalid ranges panic when visited.
impl<I: IntNumber> IntShapeResource<I> for FlatContoursBuffer<I> {
    type ResourceIter<'a>
        = IntBufferResourceIterator<'a, I>
    where
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        IntBufferResourceIterator {
            points: &self.points,
            ranges: self.ranges.iter(),
        }
    }
}

/// Visits all contour ranges in storage order, independently of `shape_ranges`.
/// Empty ranges are preserved; invalid contour ranges panic when visited.
impl<I: IntNumber> IntShapeResource<I> for FlatShapesBuffer<I> {
    type ResourceIter<'a>
        = IntBufferResourceIterator<'a, I>
    where
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        IntBufferResourceIterator {
            points: &self.points,
            ranges: self.contour_ranges.iter(),
        }
    }
}
