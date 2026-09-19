use super::resource::IntShapeResource;
pub use crate::source::iterators::ShapeResourceIterator;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

impl<I: IntNumber> IntShapeResource<I> for [Vec<IntPoint<I>>] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, IntPoint<I>>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator::with_slice(self)
    }
}

impl<I: IntNumber> IntShapeResource<I> for [&[IntPoint<I>]] {
    type ResourceIter<'a>
        = core::iter::Copied<core::slice::Iter<'a, &'a [IntPoint<I>]>>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        self.iter().copied()
    }
}
