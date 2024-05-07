use i_float::f64_point::F64Point;
use i_float::f64_rect::F64Rect;
use crate::f64::shape::{F64Path, F64Shape};

pub trait RectInit {
    fn with_shape(shape: &[F64Path]) -> Self;
    fn with_shapes(shapes: &[F64Shape]) -> Self;
}

struct Box {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl Box {

    #[inline(always)]
    fn new() -> Self {
        Box {
            min_x: f64::MAX,
            max_x: -f64::MAX,
            min_y: f64::MAX,
            max_y: -f64::MAX,
        }
    }
    #[inline]
    fn add(&mut self, point: &F64Point) {
        self.min_x = self.min_x.min(point.x);
        self.max_x = self.max_x.max(point.x);
        self.min_y = self.min_y.min(point.y);
        self.max_y = self.max_y.max(point.y);
    }

    #[inline(always)]
    fn rect(&self) -> F64Rect {
        F64Rect::new(self.min_x, self.max_x, self.min_y, self.max_y)
    }
}

impl RectInit for F64Rect {
    fn with_shape(shape: &[F64Path]) -> Self {
        if shape.is_empty() {
            return Self {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            };
        }

        let mut b = Box::new();

        for path in shape.iter() {
            for p in path.iter() {
                b.add(p);
            }
        }

        b.rect()
    }

    fn with_shapes(shapes: &[F64Shape]) -> Self {
        if shapes.is_empty() {
            return Self {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            };
        }

        let mut b = Box::new();

        for shape in shapes.iter() {
            for path in shape.iter() {
                for p in path.iter() {
                    b.add(p);
                }
            }
        }

        b.rect()
    }
}