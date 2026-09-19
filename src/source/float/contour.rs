use super::resource::ShapeResource;
pub use crate::source::iterators::ContourResourceIterator;
use i_float::float::compatible::FloatPointCompatible;

impl<P: FloatPointCompatible> ShapeResource<P> for [P] {
    type ResourceIter<'a>
        = ContourResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::source::float::resource::ShapeResource;
    use alloc::vec;

    #[test]
    fn test_resource_fixed_array() {
        let array = [[0.0, 0.0], [0.0, 1.0]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_slice_array() {
        let array = [[0.0, 0.0], [0.0, 1.0]];

        let count = array.as_slice().iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_vec_array() {
        let array = vec![[0.0, 0.0], [0.0, 1.0]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn count_reports_remaining_contours() {
        let contour = [[0.0, 0.0], [0.0, 1.0]];
        let mut iter = contour.iter_paths();

        assert!(iter.next().is_some());
        assert_eq!(iter.count(), 0);
    }
}
