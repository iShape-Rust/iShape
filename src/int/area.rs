use crate::int::shape::{IntContour, IntShape};
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::point::IntPoint;

pub trait UnsafeArea<I: IntNumber>: Iterator<Item = IntPoint<I>> + Sized {
    /// Returns the signed double area of the path.
    ///
    /// The result is positive for a counter-clockwise path and negative for a
    /// clockwise path. A non-empty simple path whose coordinates satisfy the
    /// conservative range documented by `i_float::int::point::IntPoint` fits
    /// in `I::Wide`. Paths with self-intersections or repeated winding require
    /// a separate bound on the accumulated area.
    ///
    /// # Panics
    ///
    /// Panics if the iterator is empty.
    fn unsafe_area(self) -> I::Wide;
}

impl<I, T> UnsafeArea<I> for T
where
    I: IntNumber,
    T: Iterator<Item = IntPoint<I>>,
{
    fn unsafe_area(mut self) -> I::Wide {
        let first = self.next().expect("path iterator must not be empty");
        let mut previous = first;
        let mut area = I::Wide::ZERO;

        for point in self {
            let a = previous.x.to_wide().wrapping_mul(point.y.to_wide());
            let b = previous.y.to_wide().wrapping_mul(point.x.to_wide());
            area = area.wrapping_add(a).wrapping_sub(b);
            previous = point;
        }

        let a = previous.x.to_wide().wrapping_mul(first.y.to_wide());
        let b = previous.y.to_wide().wrapping_mul(first.x.to_wide());
        area.wrapping_add(a).wrapping_sub(b)
    }
}

pub trait Area<I: IntNumber> {
    fn area_two(&self) -> I::Wide;
    fn area(&self) -> I::Wide;
}

impl<I: IntNumber> Area<I> for [IntPoint<I>] {
    #[inline]
    fn area_two(&self) -> I::Wide {
        self.iter().copied().unsafe_area()
    }

    #[inline]
    fn area(&self) -> I::Wide {
        self.area_two() / I::Wide::TWO
    }
}

impl<I: IntNumber> Area<I> for [IntContour<I>] {
    #[inline]
    fn area_two(&self) -> I::Wide {
        let mut s = I::Wide::ZERO;
        for path in self.iter() {
            s = s.wrapping_add(path.area_two())
        }
        s
    }

    #[inline]
    fn area(&self) -> I::Wide {
        self.area_two() / I::Wide::TWO
    }
}

impl<I: IntNumber> Area<I> for [IntShape<I>] {
    #[inline]
    fn area_two(&self) -> I::Wide {
        let mut s = I::Wide::ZERO;
        for shape in self.iter() {
            s = s.wrapping_add(shape.area_two())
        }
        s
    }

    #[inline]
    fn area(&self) -> I::Wide {
        self.area_two() / I::Wide::TWO
    }
}

#[cfg(test)]
mod tests {
    use crate::int::area::Area;
    use crate::int_path;

    #[test]
    fn test_0() {
        let square = int_path![[-1, -1], [1, -1], [1, 1], [-1, 1]];

        let area = square.area_two();
        assert_eq!(area, 8i64);
    }
}
