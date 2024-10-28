use i_float::adapter::FloatPointAdapter;
use i_float::float::Float;
use crate::float::shape::{FloatPath, FloatShape, FloatShapes};
use crate::int::path::IntPath;
use crate::int::shape::{IntShape, IntShapes};

pub trait PathToFloat<T: Float> {
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> FloatPath<T>;
}

pub trait ShapeToFloat<T: Float> {
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> FloatShape<T>;
}

pub trait ShapesToFloat<T: Float> {
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> FloatShapes<T>;
}

pub trait PathToInt<T: Float> {
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntPath;
}

pub trait ShapeToInt<T: Float> {
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShape;
}

pub trait ShapesToInt<T: Float> {
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShapes;
}

impl<T: Float> PathToFloat<T> for IntPath {
    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> FloatPath<T> {
        self.iter().map(|&p| adapter.convert_to_float(p)).collect()
    }
}

impl<T: Float> ShapeToFloat<T> for IntShape {

    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> FloatShape<T> {
        self.iter().map(|path| path.to_float(adapter)).collect()
    }
}

impl<T: Float> ShapesToFloat<T> for IntShapes {

    #[inline(always)]
    fn to_float(&self, adapter: &FloatPointAdapter<T>) -> FloatShapes<T> {
        self.iter().map(|shape| shape.to_float(adapter)).collect()
    }
}

impl<T: Float> PathToInt<T> for FloatPath<T> {

    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntPath {
        self.iter().map(|&p| adapter.convert_to_int(p)).collect()
    }
}

impl<T: Float> ShapeToInt<T> for FloatShape<T> {

    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShape {
        self.iter().map(|path| path.to_int(adapter)).collect()
    }
}

impl<T: Float> ShapesToInt<T> for FloatShapes<T> {

    #[inline(always)]
    fn to_int(&self, adapter: &FloatPointAdapter<T>) -> IntShapes {
        self.iter().map(|shape| shape.to_int(adapter)).collect()
    }
}