use i_float::f32_point::F32Point;
use crate::int::shape::PointsCount;

pub type F32Path = Vec<F32Point>;
pub type F32Shape = Vec<F32Path>;
pub type F32Shapes = Vec<F32Shape>;

impl PointsCount for [F32Path] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, path| acc + path.len())
    }
}

impl PointsCount for [F32Shape] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, shape| acc + shape.points_count())
    }
}