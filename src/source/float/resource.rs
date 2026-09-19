use alloc::vec::Vec;
use i_float::float::compatible::FloatPointCompatible;

/// Repeatable, borrowed traversal of paths in storage order.
///
/// Each call starts a new traversal and yields slices of the source points without
/// allocating or copying them. Empty paths are preserved; empty collections yield
/// no paths. Nested shapes are flattened, so shape boundaries are not retained.
///
/// This interface imposes no winding, closure, minimum length, or geometric
/// validity requirements. These belong to the consuming operation. In particular,
/// path order does not identify outer boundaries or holes.
pub trait ShapeResource<P: FloatPointCompatible> {
    type ResourceIter<'a>: Iterator<Item = &'a [P]>
    where
        P: 'a,
        Self: 'a;

    fn iter_paths(&self) -> Self::ResourceIter<'_>;
}

impl<P: FloatPointCompatible, T> ShapeResource<P> for Vec<T>
where
    [T]: ShapeResource<P>,
{
    type ResourceIter<'a>
        = <[T] as ShapeResource<P>>::ResourceIter<'a>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <[T] as ShapeResource<P>>::iter_paths(self.as_slice())
    }
}

impl<P: FloatPointCompatible, T, const N: usize> ShapeResource<P> for [T; N]
where
    [T]: ShapeResource<P>,
{
    type ResourceIter<'a>
        = <[T] as ShapeResource<P>>::ResourceIter<'a>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <[T] as ShapeResource<P>>::iter_paths(self.as_slice())
    }
}

impl<P: FloatPointCompatible, R: ShapeResource<P> + ?Sized> ShapeResource<P> for &R {
    type ResourceIter<'a>
        = <R as ShapeResource<P>>::ResourceIter<'a>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <R as ShapeResource<P>>::iter_paths(&**self)
    }
}

impl<P: FloatPointCompatible, R: ShapeResource<P> + ?Sized> ShapeResource<P> for &mut R {
    type ResourceIter<'a>
        = <R as ShapeResource<P>>::ResourceIter<'a>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        <R as ShapeResource<P>>::iter_paths(&**self)
    }
}
