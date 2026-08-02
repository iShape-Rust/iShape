use crate::source::resource::ShapeResource;
use alloc::vec::Vec;
use i_float::float::compatible::FloatPointCompatible;

pub struct ShapeResourceIterator<'a, P> {
    slice: &'a [Vec<P>],
    index: usize,
}

impl<'a, P> ShapeResourceIterator<'a, P> {
    #[inline]
    fn with_slice(slice: &'a [Vec<P>]) -> Self {
        Self { slice, index: 0 }
    }
}

impl<'a, P> Iterator for ShapeResourceIterator<'a, P> {
    type Item = &'a [P];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.slice.len() {
            return None;
        }
        let i = self.index;
        self.index += 1;
        self.slice.get(i).map(Vec::as_slice)
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.slice.len().saturating_sub(self.index)
    }
}

impl<P> ShapeResource<P> for [Vec<P>]
where
    P: FloatPointCompatible,
{
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

impl<P, const N: usize> ShapeResource<P> for [Vec<P>; N]
where
    P: FloatPointCompatible,
{
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

impl<P> ShapeResource<P> for Vec<Vec<P>>
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator::with_slice(self.as_slice())
    }
}

impl<'b, P> ShapeResource<P> for &'b [Vec<P>]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'b> {
        ShapeResourceIterator::with_slice(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::source::resource::ShapeResource;
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
