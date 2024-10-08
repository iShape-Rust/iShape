use i_float::f64_point::F64Point;
use crate::int::shape::PointsCount;

pub type F64Path = Vec<F64Point>;
pub type F64Shape = Vec<F64Path>;
pub type F64Shapes = Vec<F64Shape>;

impl PointsCount for [F64Path] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, path| acc + path.len())
    }
}

impl PointsCount for [F64Shape] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, shape| acc + shape.points_count())
    }
}