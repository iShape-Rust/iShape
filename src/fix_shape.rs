use crate::fix_path::{FixPath, FixPathExtension };

/// Represents a fixed geometric shape with contour and holes.
pub struct FixShape {
    paths: Vec<FixPath>,
}

impl FixShape {
    /// Returns the contour defining the outer boundary of the shape.
    pub fn contour(&self) -> &FixPath {
        &self.paths[0]
    }

    /// Returns the array of holes defining the inner boundaries of the shape.
    pub fn holes(&self) -> &[FixPath] {
        &self.paths[1..]
    }

    /// Initializes a new shape with the specified contour and holes.
    pub fn new_with_contour_and_holes(contour: FixPath, holes: Vec<FixPath>) -> Self {
        let mut paths = Vec::with_capacity(holes.len() + 1);
        paths.push(contour);
        paths.extend(holes);
        Self { paths }
    }

    /// Initializes a new shape with the specified paths.
    /// The first path is used as the contour, and remaining paths as holes.
    pub fn new(paths: Vec<FixPath>) -> Self {
        Self { paths }
    }

    /// Sets the direction of the contour and holes.
    /// If the clockwise parameter is true, the contour and holes will be arranged in a clockwise direction.
    /// If false, they will be arranged in a counter-clockwise direction.
    pub fn set_direction(&mut self, clockwise: bool) {
        for path in &mut self.paths {
            if (path.area() < 0) == clockwise {
                path.reverse();
            }
        }
    }

    /// Adds a new hole to the shape.
    pub fn add_hole(&mut self, path: FixPath) {
        self.paths.push(path);
    }
}