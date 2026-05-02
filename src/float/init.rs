use crate::base::data::Contour;
use crate::int::shape::IntContour;
use crate::util::reserve::Reserve;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;

pub trait IntContourInit<P: FloatPointCompatible> {
    fn set_with_float(&mut self, contour: &Contour<P>, adapter: &FloatPointAdapter<P>);
}

impl<P: FloatPointCompatible> IntContourInit<P> for IntContour {
    fn set_with_float(&mut self, contour: &Contour<P>, adapter: &FloatPointAdapter<P>) {
        self.reserve_capacity(contour.len());
        self.clear();
        for p in contour.iter() {
            self.push(adapter.float_to_int(p))
        }
    }
}
