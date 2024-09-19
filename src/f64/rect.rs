use i_float::f64_point::F64Point;
use i_float::f64_rect::F64Rect;
use crate::f64::shape::{F64Path, F64Shape};

pub trait RectInit {
    fn with_shape(shape: &[F64Path]) -> Option<F64Rect>;
    fn with_shapes(shapes: &[F64Shape]) -> Option<F64Rect>;
}

trait FirstPoint {
    fn first_point(&self) -> Option<F64Point>;
}

impl RectInit for F64Rect {
    fn with_shape(shape: &[F64Path]) -> Option<F64Rect> {
        let first_point = shape.first_point()?;

        let mut rect = Self {
            min_x: first_point.x,
            max_x: first_point.x,
            min_y: first_point.y,
            max_y: first_point.y,
        };

        for path in shape.iter() {
            for p in path.iter() {
                rect.unsafe_add_point(p);
            }
        }

        Some(rect)
    }

    fn with_shapes(shapes: &[F64Shape]) -> Option<F64Rect> {
        let first_point = shapes.first_point()?;

        let mut rect = Self {
            min_x: first_point.x,
            max_x: first_point.x,
            min_y: first_point.y,
            max_y: first_point.y,
        };

        for shape in shapes.iter() {
            for path in shape.iter() {
                for p in path.iter() {
                    rect.unsafe_add_point(p);
                }
            }
        }

        Some(rect)
    }
}

impl FirstPoint for [F64Path] {
    fn first_point(&self) -> Option<F64Point> {
        for path in self.iter() {
            if let Some(p) = path.first() {
                return Some(*p);
            }
        }
        None
    }
}

impl FirstPoint for [F64Shape] {
    fn first_point(&self) -> Option<F64Point> {
        for shape in self.iter() {
            if let Some(p) = shape.first_point() {
                return Some(p);
            }
        }
        None
    }
}