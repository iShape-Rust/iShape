use serde::{Serialize, Deserialize};
use crate::fix_path::{FixPath, FixPathExtension};
use crate::fix_paths::FixPathsExtension;

/// Represents a fixed geometric shape with contour and holes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FixShape {
    pub paths: Vec<FixPath>,
}

impl FixShape {

    pub fn points_count(&self) -> usize {
        self.paths.points_count()
    }
    /// Determines if the shape represents a convex polygon.
    ///
    /// # Returns
    /// `true` if the shape is a convex polygon, `false` otherwise.
    pub fn is_convex_polygon(&self) -> bool {
        self.paths.len() == 1 && self.contour().is_convex()
    }

    /// Returns the contour defining the outer boundary of the shape.
    /// Assumes the first path in `paths` is the contour.
    ///
    /// # Returns
    /// A reference to the `FixPath` representing the contour.
    pub fn contour(&self) -> &FixPath {
        &self.paths[0]
    }

    /// Returns the array of holes defining the inner boundaries of the shape.
    ///
    /// # Returns
    /// A slice of `FixPath` representing the holes.
    pub fn holes(&self) -> &[FixPath] {
        &self.paths[1..]
    }

    /// Initializes a new shape with the specified contour.
    /// Automatically adjusts the order of the contour to be clockwise.
    ///
    /// # Parameters
    /// * `contour`: The `FixPath` defining the outer boundary of the shape.
    ///
    /// # Returns
    /// A new instance of `FixShape`.
    pub fn new_with_contour(contour: FixPath) -> Self {
        Self { paths: vec![if contour.is_clockwise_ordered() { contour } else { contour.into_reversed() }] }
    }

    /// Initializes a new shape with the specified contour and holes.
    /// Adjusts the order of the contour to be clockwise and holes to be counter-clockwise.
    ///
    /// # Parameters
    /// * `contour`: The `FixPath` defining the outer boundary of the shape.
    /// * `holes`: A `Vec<FixPath>` defining the inner boundaries of the shape.
    ///
    /// # Returns
    /// A new instance of `FixShape`.
    pub fn new_with_contour_and_holes(contour: FixPath, holes: Vec<FixPath>) -> Self {
        let mut paths = Vec::with_capacity(holes.len() + 1);
        paths.push(if contour.is_clockwise_ordered() { contour } else { contour.into_reversed() });

        for hole in holes {
            paths.push(if hole.is_clockwise_ordered() { hole.into_reversed() } else { hole });
        }

        Self { paths }
    }

    /// Initializes a new shape with the specified paths.
    /// The first path is used as the contour, and remaining paths as holes.
    /// Assumes that paths are properly ordered beforehand.
    ///
    /// # Parameters
    /// * `paths`: A `Vec<FixPath>` defining the contour and holes.
    ///
    /// # Returns
    /// A new instance of `FixShape`.
    pub fn new(paths: Vec<FixPath>) -> Self {
        Self { paths }
    }

    /// Adds a new hole to the shape.
    /// Automatically adjusts the order of the hole to be counter-clockwise.
    ///
    /// # Parameters
    /// * `path`: The `FixPath` defining the hole to be added.
    pub fn add_hole(&mut self, path: FixPath) {
        self.paths.push(if path.is_clockwise_ordered() { path.into_reversed() } else { path });
    }
}