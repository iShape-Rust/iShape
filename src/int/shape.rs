use crate::int::path::IntPath;

pub type IntShape = Vec<IntPath>;
pub type IntShapes = Vec<IntShape>;

pub trait PointsCount {
    fn points_count(&self) -> usize;
}

impl PointsCount for [IntPath] {
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, path| acc + path.len())
    }
}

impl PointsCount for [IntShape] {
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, shape| acc + shape.points_count())
    }
}