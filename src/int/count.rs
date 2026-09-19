use crate::int::path::IntPath;
use crate::int::shape::IntShape;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;

pub type IntShapes<I> = Vec<IntShape<I>>;

pub trait PointsCount {
    fn points_count(&self) -> usize;
}

impl<I: IntNumber> PointsCount for [IntPath<I>] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, path| acc + path.len())
    }
}

impl<I: IntNumber> PointsCount for [IntShape<I>] {
    #[inline(always)]
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, shape| acc + shape.points_count())
    }
}
