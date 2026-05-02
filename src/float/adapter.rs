use crate::base::data::{Contour, Path, Shape, Shapes};
use crate::flat::buffer::FlatContoursBuffer;
use crate::flat::float::FloatFlatContoursBuffer;
use crate::int::path::IntPath;
use crate::int::shape::{IntContour, IntShape, IntShapes};
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::int::point::IntPoint;

pub trait PathToFloat<P: FloatPointCompatible> {
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> Path<P>;
}

pub trait ShapeToFloat<P: FloatPointCompatible> {
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> Shape<P>;
}

pub trait ShapesToFloat<P: FloatPointCompatible> {
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> Shapes<P>;
}

pub trait BufferToFloat<P: FloatPointCompatible> {
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> FloatFlatContoursBuffer<P>;
}

pub trait PathToInt<P: FloatPointCompatible> {
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> IntPath;
}

pub trait ShapeToInt<P: FloatPointCompatible> {
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> IntShape;
}

pub trait ShapesToInt<P: FloatPointCompatible> {
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> IntShapes;
}

pub trait BufferToInt<P: FloatPointCompatible> {
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> FlatContoursBuffer;
}

impl<P: FloatPointCompatible> PathToFloat<P> for [IntPoint] {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> Path<P> {
        self.iter().map(|p| adapter.int_to_float(p)).collect()
    }
}

impl<P: FloatPointCompatible> ShapeToFloat<P> for [IntContour] {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> Shape<P> {
        self.iter().map(|path| path.to_float(adapter)).collect()
    }
}

impl<P: FloatPointCompatible> ShapesToFloat<P> for [IntShape] {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> Shapes<P> {
        self.iter().map(|shape| shape.to_float(adapter)).collect()
    }
}

impl<P: FloatPointCompatible> BufferToFloat<P> for FlatContoursBuffer {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<P>) -> FloatFlatContoursBuffer<P> {
        FloatFlatContoursBuffer {
            points: self.points.to_float(adapter),
            ranges: self.ranges.clone(),
        }
    }
}

impl<P: FloatPointCompatible> PathToInt<P> for [P] {
    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> IntPath {
        self.iter().map(|p| adapter.float_to_int(p)).collect()
    }
}

impl<P: FloatPointCompatible> ShapeToInt<P> for [Contour<P>] {
    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> IntShape {
        self.iter().map(|path| path.to_int(adapter)).collect()
    }
}

impl<P: FloatPointCompatible> ShapesToInt<P> for [Shape<P>] {
    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> IntShapes {
        self.iter().map(|shape| shape.to_int(adapter)).collect()
    }
}

impl<P: FloatPointCompatible> BufferToInt<P> for FloatFlatContoursBuffer<P> {
    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<P>) -> FlatContoursBuffer {
        FlatContoursBuffer {
            points: self.points.to_int(adapter),
            ranges: self.ranges.clone(),
        }
    }
}
