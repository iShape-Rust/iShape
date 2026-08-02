use crate::source::resource::ShapeResource;
use alloc::vec::Vec;
use i_float::float::compatible::FloatPointCompatible;

pub struct ShapesResourceIterator<'a, P> {
    slice: &'a [Vec<Vec<P>>],
    i: usize,
    j: usize,
}

impl<'a, P> ShapesResourceIterator<'a, P> {
    #[inline]
    fn with_slice(slice: &'a [Vec<Vec<P>>]) -> Self {
        Self { slice, i: 0, j: 0 }
    }
}

impl<'a, P> Iterator for ShapesResourceIterator<'a, P> {
    type Item = &'a [P];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.slice.len() {
            let sub_slice = self.slice.get(self.i)?;
            if self.j < sub_slice.len() {
                let j = self.j;
                self.j += 1;
                return sub_slice.get(j).map(Vec::as_slice);
            }
            self.i += 1;
            self.j = 0;
        }

        None
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        let Some((current, remaining)) = self.slice.get(self.i..).and_then(|slice| slice.split_first())
        else {
            return 0;
        };

        current.len().saturating_sub(self.j) + remaining.iter().map(Vec::len).sum::<usize>()
    }
}

impl<P> ShapeResource<P> for [Vec<Vec<P>>]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapesResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ShapesResourceIterator::with_slice(self)
    }
}

impl<P, const N: usize> ShapeResource<P> for [Vec<Vec<P>>; N]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapesResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ShapesResourceIterator::with_slice(self)
    }
}

impl<P> ShapeResource<P> for Vec<Vec<Vec<P>>>
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapesResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        ShapesResourceIterator::with_slice(self.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use crate::source::resource::ShapeResource;
    use alloc::vec;

    #[test]
    fn test_resource_fixed_array() {
        let array = [vec![vec![[0.0, 0.0], [0.0, 1.0]]]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_slice_array() {
        let array = [vec![vec![[0.0, 0.0], [0.0, 1.0]]]];

        let count = array.as_slice().iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_vec_array() {
        let array = vec![vec![vec![[0.0, 0.0], [0.0, 1.0]]]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn count_reports_remaining_contours() {
        let shapes = [vec![vec![[0.0, 0.0]], vec![[1.0, 1.0]]], vec![vec![[2.0, 2.0]]]];
        let mut iter = shapes.iter_paths();

        assert!(iter.next().is_some());
        assert_eq!(iter.count(), 2);
    }
}
