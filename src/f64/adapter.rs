use i_float::f64_point::F64Point;
use i_float::point::IntPoint;
use crate::f64::rect::F64Rect;
use crate::f64::shape::{F64Path, F64Shape, F64Shapes};
use crate::int::path::IntPath;
use crate::int::shape::{IntShape, IntShapes};

pub struct PointAdapter {
    pub dir_scale: f64,
    pub inv_scale: f64,
    pub offset: F64Point,
}

impl PointAdapter {
    pub fn new(rect: F64Rect) -> Self {
        let a = rect.width / 2.0;
        let b = rect.height / 2.0;

        let ox = rect.x + a;
        let oy = rect.y + b;

        let offset = F64Point { x: ox, y: oy };

        let max = a.max(b);
        let log2 = max.log2() as i32;
        let e = 30 - log2;

        let dir_scale = 2f64.powi(e);
        let inv_scale = 2f64.powi(-e);

        PointAdapter {
            dir_scale,
            inv_scale,
            offset,
        }
    }

    pub fn convert_to_float(&self, point: &IntPoint) -> F64Point {
        let x = point.x as f64 * self.inv_scale + self.offset.x;
        let y = point.y as f64 * self.inv_scale + self.offset.y;
        F64Point { x, y }
    }

    pub fn convert_to_int(&self, point: &F64Point) -> IntPoint {
        let x = ((point.x - self.offset.x) * self.dir_scale) as i32;
        let y = ((point.y - self.offset.y) * self.dir_scale) as i32;
        IntPoint { x, y }
    }
}

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
    fn to_float(&self, adapter: &PointAdapter) -> F64Path {
        self.iter().map(|p| adapter.convert_to_float(p)).collect()
    }
}

impl ShapeToFloat for IntShape {
    fn to_float(&self, adapter: &PointAdapter) -> F64Shape {
        self.iter().map(|path| path.to_float(adapter)).collect()
    }
}

impl ShapesToFloat for IntShapes {
    fn to_float(&self, adapter: &PointAdapter) -> F64Shapes {
        self.iter().map(|shape| shape.to_float(adapter)).collect()
    }
}

impl PathToInt for F64Path {
    fn to_int(&self, adapter: &PointAdapter) -> IntPath {
        self.iter().map(|p| adapter.convert_to_int(p)).collect()
    }
}

impl ShapeToInt for F64Shape {
    fn to_int(&self, adapter: &PointAdapter) -> IntShape {
        self.iter().map(|path| path.to_int(adapter)).collect()
    }
}

impl ShapesToInt for F64Shapes {
    fn to_int(&self, adapter: &PointAdapter) -> IntShapes {
        self.iter().map(|shape| shape.to_int(adapter)).collect()
    }
}