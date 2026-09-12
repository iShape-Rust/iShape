use super::resource::ShapeResource;
pub use crate::source::iterators::ShapeResourceIterator;
use alloc::vec::Vec;
use i_float::float::compatible::FloatPointCompatible;

impl<P: FloatPointCompatible> ShapeResource<P> for [Vec<P>] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator::with_slice(self)
    }
}

impl<P: FloatPointCompatible> ShapeResource<P> for [&[P]] {
    type ResourceIter<'a>
        = core::iter::Copied<core::slice::Iter<'a, &'a [P]>>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        self.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::source::float::resource::ShapeResource;
    use alloc::vec;

    #[test]
    fn test_resource_fixed_array() {
        let array = [vec![[0.0, 0.0], [0.0, 1.0]]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_slice_array() {
        let array = [vec![[0.0, 0.0], [0.0, 1.0]]];

        let count = array.as_slice().iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_vec_array() {
        let array = vec![vec![[0.0, 0.0], [0.0, 1.0]]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn count_reports_remaining_contours() {
        let shape = [vec![[0.0, 0.0]], vec![[1.0, 1.0]]];
        let mut iter = shape.iter_paths();

        assert!(iter.next().is_some());
        assert_eq!(iter.count(), 1);
    }
}
