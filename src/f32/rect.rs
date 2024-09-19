use i_float::f32_point::F32Point;
use i_float::f32_rect::F32Rect;
use crate::f32::shape::{F32Path, F32Shape};

pub trait RectInit {
    fn with_shape(shape: &[F32Path]) -> Option<F32Rect>;
    fn with_shapes(shapes: &[F32Shape]) -> Option<F32Rect>;
}

trait FirstPoint {
    fn first_point(&self) -> Option<F32Point>;
}

impl RectInit for F32Rect {
    fn with_shape(shape: &[F32Path]) -> Option<F32Rect> {
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

    fn with_shapes(shapes: &[F32Shape]) -> Option<F32Rect> {
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

impl FirstPoint for [F32Path] {
    fn first_point(&self) -> Option<F32Point> {
        for path in self.iter() {
            if let Some(p) = path.first() {
                return Some(*p);
            }
        }
        None
    }
}

impl FirstPoint for [F32Shape] {
    fn first_point(&self) -> Option<F32Point> {
        for shape in self.iter() {
            if let Some(p) = shape.first_point() {
                return Some(p);
            }
        }
        None
    }
}