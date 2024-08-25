use i_float::f32_adapter::F32PointAdapter;
use crate::f32::shape::{F32Path, F32Shape, F32Shapes};
use crate::int::path::IntPath;
use crate::int::shape::{IntShape, IntShapes};


pub trait PathToFloat {
    fn to_float(&self, adapter: &F32PointAdapter) -> F32Path;
}

pub trait ShapeToFloat {
    fn to_float(&self, adapter: &F32PointAdapter) -> F32Shape;
}

pub trait ShapesToFloat {
    fn to_float(&self, adapter: &F32PointAdapter) -> F32Shapes;
}

pub trait PathToInt {
    fn to_int(&self, adapter: &F32PointAdapter) -> IntPath;
}

pub trait ShapeToInt {
    fn to_int(&self, adapter: &F32PointAdapter) -> IntShape;
}

pub trait ShapesToInt {
    fn to_int(&self, adapter: &F32PointAdapter) -> IntShapes;
}

impl PathToFloat for IntPath {

    #[inline(always)]
    fn to_float(&self, adapter: &F32PointAdapter) -> F32Path {
        self.iter().map(|p| adapter.convert_to_float(p)).collect()
    }
}

impl ShapeToFloat for IntShape {

    #[inline(always)]
    fn to_float(&self, adapter: &F32PointAdapter) -> F32Shape {
        self.iter().map(|path| path.to_float(adapter)).collect()
    }
}

impl ShapesToFloat for IntShapes {

    #[inline(always)]
    fn to_float(&self, adapter: &F32PointAdapter) -> F32Shapes {
        self.iter().map(|shape| shape.to_float(adapter)).collect()
    }
}

impl PathToInt for F32Path {

    #[inline(always)]
    fn to_int(&self, adapter: &F32PointAdapter) -> IntPath {
        self.iter().map(|p| adapter.convert_to_int(p)).collect()
    }
}

impl ShapeToInt for F32Shape {

    #[inline(always)]
    fn to_int(&self, adapter: &F32PointAdapter) -> IntShape {
        self.iter().map(|path| path.to_int(adapter)).collect()
    }
}

impl ShapesToInt for F32Shapes {

    #[inline(always)]
    fn to_int(&self, adapter: &F32PointAdapter) -> IntShapes {
        self.iter().map(|shape| shape.to_int(adapter)).collect()
    }
}