use super::resource::IntShapeResource;
pub use crate::source::iterators::ContourResourceIterator;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

impl<I: IntNumber> IntShapeResource<I> for [IntPoint<I>] {
    type ResourceIter<'a>
        = ContourResourceIterator<'a, IntPoint<I>>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self)
    }
}
