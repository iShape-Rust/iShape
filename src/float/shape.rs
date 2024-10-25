use i_float::float::Float;
use i_float::float_point::FloatPoint;
use crate::int::shape::PointsCount;

pub type FloatPath<T> = Vec<FloatPoint<T>>;
pub type FloatShape<T> = Vec<FloatPath<T>>;
pub type FloatShapes<T> = Vec<FloatShape<T>>;

impl<T: Float> PointsCount for [FloatPath<T>] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, path| acc + path.len())
    }
}

impl<T: Float> PointsCount for [FloatShape<T>] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, shape| acc + shape.points_count())
    }
}