use i_float::adapter::PointAdapter;
use crate::f64::shape::{F64Path, F64Shape, F64Shapes};
use crate::int::path::IntPath;
use crate::int::shape::{IntShape, IntShapes};


pub trait PathToFloat {
    fn to_float(&self, adapter: &PointAdapter) -> F64Path;
}

pub trait ShapeToFloat {
    fn to_float(&self, adapter: &PointAdapter) -> F64Shape;
}

pub trait ShapesToFloat {
    fn to_float(&self, adapter: &PointAdapter) -> F64Shapes;
}

pub trait PathToInt {
    fn to_int(&self, adapter: &PointAdapter) -> IntPath;
}

pub trait ShapeToInt {
    fn to_int(&self, adapter: &PointAdapter) -> IntShape;
}

pub trait ShapesToInt {
    fn to_int(&self, adapter: &PointAdapter) -> IntShapes;
}

impl PathToFloat for IntPath {

    #[inline(always)]
    fn to_float(&self, adapter: &PointAdapter) -> F64Path {
        self.iter().map(|p| adapter.convert_to_float(p)).collect()
    }
}

impl ShapeToFloat for IntShape {

    #[inline(always)]
    fn to_float(&self, adapter: &PointAdapter) -> F64Shape {
        self.iter().map(|path| path.to_float(adapter)).collect()
    }
}

impl ShapesToFloat for IntShapes {

    #[inline(always)]
    fn to_float(&self, adapter: &PointAdapter) -> F64Shapes {
        self.iter().map(|shape| shape.to_float(adapter)).collect()
    }
}

impl PathToInt for F64Path {

    #[inline(always)]
    fn to_int(&self, adapter: &PointAdapter) -> IntPath {
        self.iter().map(|p| adapter.convert_to_int(p)).collect()
    }
}

impl ShapeToInt for F64Shape {

    #[inline(always)]
    fn to_int(&self, adapter: &PointAdapter) -> IntShape {
        self.iter().map(|path| path.to_int(adapter)).collect()
    }
}

impl ShapesToInt for F64Shapes {

    #[inline(always)]
    fn to_int(&self, adapter: &PointAdapter) -> IntShapes {
        self.iter().map(|shape| shape.to_int(adapter)).collect()
    }
}