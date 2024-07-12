use i_float::f64_rect::F64Rect;
use crate::f64::shape::{F64Path, F64Shape};

pub trait RectInit {
    fn with_shape(shape: &[F64Path]) -> Option<F64Rect>;
    fn with_shapes(shapes: &[F64Shape]) -> Option<F64Rect>;
}

impl RectInit for F64Rect {
    fn with_shape(shape: &[F64Path]) -> Option<F64Rect> {
        if shape.is_empty() {
            return None;
        }

        let mut rect = if let Some(first_point) = shape.iter()
            .flat_map(|path| path.iter())
            .next() {
            F64Rect {
                min_x: first_point.x,
                max_x: first_point.x,
                min_y: first_point.y,
                max_y: first_point.y,
            }
        } else {
            return None;
        };

        for path in shape.iter() {
            for p in path.iter() {
                rect.add_point(p);
            }
        }

        Some(rect)
    }

    fn with_shapes(shapes: &[F64Shape]) -> Option<F64Rect> {
        if shapes.is_empty() {
            return None;
        }

        let mut rect = if let Some(first_point) = shapes.iter()
            .flat_map(|shape| shape.iter())
            .flat_map(|path| path.iter())
            .next() {
            Self {
                min_x: first_point.x,
                max_x: first_point.x,
                min_y: first_point.y,
                max_y: first_point.y,
            }
        } else {
            return None;
        };

        for shape in shapes.iter() {
            for path in shape.iter() {
                for p in path.iter() {
                    rect.add_point(p);
                }
            }
        }

        Some(rect)
    }
}