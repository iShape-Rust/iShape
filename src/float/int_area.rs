use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;

pub trait IntArea<P: FloatPointCompatible, I: IntNumber> {
    /// The area of the `Path`.
    /// - Returns: A positive double area if path is counter-clockwise and negative double area otherwise.
    fn unsafe_int_area(&self, adapter: &FloatPointAdapter<P, I>) -> I::Wide;
}

impl<P: FloatPointCompatible, I: IntNumber> IntArea<P, I> for [P] {
    fn unsafe_int_area(&self, adapter: &FloatPointAdapter<P, I>) -> I::Wide {
        let Some(last) = self.last() else {
            return I::Wide::ZERO;
        };
        let mut p0 = adapter.float_to_int(last);
        let mut area = I::Wide::ZERO;

        for pi in self.iter() {
            let p1 = adapter.float_to_int(pi);
            let a = p0.x.to_wide().wrapping_mul(p1.y.to_wide());
            let b = p0.y.to_wide().wrapping_mul(p1.x.to_wide());
            area = area.wrapping_add(a).wrapping_sub(b);
            p0 = p1;
        }

        area
    }
}

#[cfg(test)]
mod tests {
    use crate::float::int_area::IntArea;
    use crate::path;
    use alloc::vec::Vec;
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;

    #[test]
    fn test_0() {
        let square = path![[-1f32, -1f32], [1f32, -1f32], [1f32, 1f32], [-1f32, 1f32],];
        let adapter = FloatPointAdapter::<_, i32>::with_iter(square.iter());

        let area = square.unsafe_int_area(&adapter);
        assert!(area > 0i64);
    }

    #[test]
    fn empty_contour_has_zero_area() {
        let contour = Vec::<[f32; 2]>::new();
        let adapter = FloatPointAdapter::<_, i32>::new(FloatRect::zero());

        assert_eq!(contour.unsafe_int_area(&adapter), 0);
    }
}
