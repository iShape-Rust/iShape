use i_float::f64_rect::F64Rect;
use crate::f64::shape::{F64Path, F64Shape};

pub trait RectInit {
    fn with_shape(shape: &[F64Path]) -> Self;
    fn with_shapes(shapes: &[F64Shape]) -> Self;
}

impl RectInit for F64Rect {
    fn with_shape(shape: &[F64Path]) -> Self {
        if shape.is_empty() {
            return Self { min_x: -f64::MAX, max_x: -f64::MAX, min_y: -f64::MAX, max_y: -f64::MAX };
        }

        let mut rect = Self { min_x: f64::MAX, max_x: -f64::MAX, min_y: f64::MAX, max_y: -f64::MAX };

        for path in shape.iter() {
            for p in path.iter() {
                rect.add_point(p);
            }
        }

        rect
    }

    fn with_shapes(shapes: &[F64Shape]) -> Self {
        if shapes.is_empty() {
            return Self { min_x: -f64::MAX, max_x: -f64::MAX, min_y: -f64::MAX, max_y: -f64::MAX };
        }

        let mut rect = Self { min_x: f64::MAX, max_x: -f64::MAX, min_y: f64::MAX, max_y: -f64::MAX };

        for shape in shapes.iter() {
            for path in shape.iter() {
                for p in path.iter() {
                    rect.add_point(p);
                }
            }
        }

        rect
    }
}