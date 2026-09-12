use alloc::vec::Vec;

pub struct ContourResourceIterator<'a, P> {
    slice: &'a [P],
    finished: bool,
}

impl<'a, P> ContourResourceIterator<'a, P> {
    #[inline]
    pub(crate) fn with_slice(slice: &'a [P]) -> Self {
        Self {
            slice,
            finished: false,
        }
    }
}

impl<'a, P> Iterator for ContourResourceIterator<'a, P> {
    type Item = &'a [P];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        self.finished = true;
        Some(self.slice)
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        usize::from(!self.finished)
    }
}

pub struct ShapeResourceIterator<'a, P> {
    slice: &'a [Vec<P>],
    index: usize,
}

impl<'a, P> ShapeResourceIterator<'a, P> {
    #[inline]
    pub(crate) fn with_slice(slice: &'a [Vec<P>]) -> Self {
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

pub struct ShapesResourceIterator<'a, P> {
    slice: &'a [Vec<Vec<P>>],
    i: usize,
    j: usize,
}

impl<'a, P> ShapesResourceIterator<'a, P> {
    #[inline]
    pub(crate) fn with_slice(slice: &'a [Vec<Vec<P>>]) -> Self {
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
