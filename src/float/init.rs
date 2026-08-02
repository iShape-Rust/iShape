use crate::base::data::Contour;
use crate::int::shape::IntContour;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::int::number::int::IntNumber;

pub trait IntContourInit<P: FloatPointCompatible, I: IntNumber> {
    fn set_with_float(&mut self, contour: &Contour<P>, adapter: &FloatPointAdapter<P, I>);
}

impl<P: FloatPointCompatible, I: IntNumber> IntContourInit<P, I> for IntContour<I> {
    fn set_with_float(&mut self, contour: &Contour<P>, adapter: &FloatPointAdapter<P, I>) {
        self.clear();
        self.extend(contour.iter().map(|p| adapter.float_to_int(p)));
    }
}
