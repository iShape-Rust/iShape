use super::resource::IntShapeResource;
pub use crate::source::iterators::ShapesResourceIterator;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

impl<I: IntNumber> IntShapeResource<I> for [Vec<Vec<IntPoint<I>>>] {
    type ResourceIter<'a>
        = ShapesResourceIterator<'a, IntPoint<I>>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ShapesResourceIterator::with_slice(self)
    }
}
