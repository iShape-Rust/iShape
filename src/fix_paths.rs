use crate::fix_path::FixPath;

pub type FixPaths = Vec<FixPath>;

pub trait FixPathsExtension {
    fn points_count(&self) -> usize;
}

impl FixPathsExtension for FixPaths {
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, x| acc + x.len())
    }
}

impl FixPathsExtension for [FixPath] {
    fn points_count(&self) -> usize {
        self.iter().fold(0, |acc, x| acc + x.len())
    }
}