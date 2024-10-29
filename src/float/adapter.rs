use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::base::data::{Contour, Shape, Shapes};
use crate::int::path::IntPath;
use crate::int::shape::{IntShape, IntShapes};

pub trait PathToFloat<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> Contour<P>;
}

pub trait ShapeToFloat<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> Shape<P>;
}

pub trait ShapesToFloat<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> Shapes<P>;
}

pub trait PathToInt<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntPath;
}

pub trait ShapeToInt<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShape;
}

pub trait ShapesToInt<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShapes;
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> PathToFloat<P, T> for IntPath {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> Contour<P> {
        self.iter().map(|&p| adapter.int_to_float(p)).collect()
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> ShapeToFloat<P, T> for IntShape {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> Shape<P> {
        self.iter().map(|path| path.to_float(adapter)).collect()
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> ShapesToFloat<P, T> for IntShapes {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> Shapes<P> {
        self.iter().map(|shape| shape.to_float(adapter)).collect()
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> PathToInt<P, T> for Contour<P> {
    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntPath {
        self.iter().map(|&p| adapter.float_to_int(p)).collect()
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> ShapeToInt<P, T> for Shape<P> {
    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShape {
        self.iter().map(|path| path.to_int(adapter)).collect()
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> ShapesToInt<P, T> for Shapes<P> {
    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShapes {
        self.iter().map(|shape| shape.to_int(adapter)).collect()
    }
}