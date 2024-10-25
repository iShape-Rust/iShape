use i_float::float::Float;
use i_float::float_point::FloatPoint;
use i_float::float_rect::FloatRect;
use crate::float::shape::{FloatPath, FloatShape};

pub trait RectInit<T: Float> {
    fn with_shape(shape: &[FloatPath<T>]) -> Option<FloatRect<T>>;
    fn with_shapes(shapes: &[FloatShape<T>]) -> Option<FloatRect<T>>;
}

trait FirstPoint<T: Float> {
    fn first_point(&self) -> Option<FloatPoint<T>>;
}

impl<T: Float> RectInit<T> for FloatRect<T> {
    fn with_shape(shape: &[FloatPath<T>]) -> Option<FloatRect<T>> {
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

    fn with_shapes(shapes: &[FloatShape<T>]) -> Option<FloatRect<T>> {
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

impl<T: Float> FirstPoint<T> for [FloatPath<T>] {
    fn first_point(&self) -> Option<FloatPoint<T>> {
        for path in self.iter() {
            if let Some(p) = path.first() {
                return Some(*p);
            }
        }
        None
    }
}

impl<T: Float> FirstPoint<T> for [FloatShape<T>] {
    fn first_point(&self) -> Option<FloatPoint<T>> {
        for shape in self.iter() {
            if let Some(p) = shape.first_point() {
                return Some(p);
            }
        }
        None
    }
}