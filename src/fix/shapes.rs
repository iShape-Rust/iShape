use crate::fix::shape::FixShape;

pub type FixShapes = Vec<FixShape>;

pub trait FixShapesExtension {
    fn points_count(&self) -> usize;
}

impl FixShapesExtension for FixShapes {
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, x| acc + x.points_count())
    }
}

impl FixShapesExtension for [FixShape] {
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, x| acc + x.points_count())
    }
}