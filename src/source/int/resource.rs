use alloc::vec::Vec;
use i_float::int::{number::int::IntNumber, point::IntPoint};

/// Repeatable, borrowed traversal of paths in storage order.
///
/// Each call starts a new traversal and yields slices of the source points without
/// allocating or copying them. Empty paths are preserved; empty collections yield
/// no paths. Nested shapes are flattened, so shape boundaries are not retained.
///
/// This interface imposes no winding, closure, minimum length, or geometric
/// validity requirements. These belong to the consuming operation. In particular,
/// path order does not identify outer boundaries or holes.
pub trait IntShapeResource<I: IntNumber> {
    type ResourceIter<'a>: Iterator<Item = &'a [IntPoint<I>]>
    where
        I: 'a,
        Self: 'a;

    fn iter_paths(&self) -> Self::ResourceIter<'_>;
}

impl<I: IntNumber, T> IntShapeResource<I> for Vec<T>
where
    [T]: IntShapeResource<I>,
{
    type ResourceIter<'a>
        = <[T] as IntShapeResource<I>>::ResourceIter<'a>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <[T] as IntShapeResource<I>>::iter_paths(self.as_slice())
    }
}

impl<I: IntNumber, T, const N: usize> IntShapeResource<I> for [T; N]
where
    [T]: IntShapeResource<I>,
{
    type ResourceIter<'a>
        = <[T] as IntShapeResource<I>>::ResourceIter<'a>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <[T] as IntShapeResource<I>>::iter_paths(self.as_slice())
    }
}

impl<I: IntNumber, R: IntShapeResource<I> + ?Sized> IntShapeResource<I> for &R {
    type ResourceIter<'a>
        = <R as IntShapeResource<I>>::ResourceIter<'a>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <R as IntShapeResource<I>>::iter_paths(&**self)
    }
}

impl<I: IntNumber, R: IntShapeResource<I> + ?Sized> IntShapeResource<I> for &mut R {
    type ResourceIter<'a>
        = <R as IntShapeResource<I>>::ResourceIter<'a>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <R as IntShapeResource<I>>::iter_paths(&**self)
    }
}
